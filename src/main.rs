use std::{fs::File, io::BufReader, str::FromStr};

use anyhow::Result;
use clap::Parser;
use console::style;
use solana_sdk::address_lookup_table::{
    instruction::{
        close_lookup_table, create_lookup_table, deactivate_lookup_table, extend_lookup_table,
    },
    state::LOOKUP_TABLE_META_SIZE,
};
use solana_sdk::{pubkey::Pubkey, signer::Signer};

mod args;
mod setup;
mod utils;

use utils::send_transaction;

fn main() -> Result<()> {
    solana_logger::setup_with_default("error");

    let args = args::Args::parse();

    match args.command {
        args::Commands::Create => create_lut()?,
        args::Commands::Extend {
            lut,
            addresses,
            file,
        } => extend_lut(lut, addresses, file)?,
        args::Commands::Close { lut } => close_lut(lut)?,
        args::Commands::Deactivate { lut } => deactivate_lut(lut)?,
        args::Commands::Decode { lut } => decode_lut(lut)?,
    }

    Ok(())
}

fn create_lut() -> Result<()> {
    let config = setup::CliConfig::new()?;

    let (ix, pda) = create_lookup_table(
        config.keypair.pubkey(),
        config.keypair.pubkey(),
        config.recent_slot,
    );

    let tx = solana_sdk::transaction::Transaction::new_signed_with_payer(
        &[ix],
        Some(&config.keypair.pubkey()),
        &[&config.keypair],
        config.recent_blockhash,
    );

    let signature = config
        .client
        .send_and_confirm_transaction_with_spinner(&tx)?;

    println!(
        "Created LUT at {} with signature {}",
        style(pda).bold().green(),
        style(signature).bold().green()
    );

    Ok(())
}

fn extend_lut(
    lut_address: String,
    addresses: Option<Vec<String>>,
    file: Option<String>,
) -> Result<()> {
    let config = setup::CliConfig::new()?;
    let authority_pubkey = config.keypair.pubkey();

    let lut_pubkey = Pubkey::from_str(&lut_address)?;

    // If neither option is provided, fail
    if addresses.is_none() && file.is_none() {
        return Err(anyhow::anyhow!(
            "Must provide some addresses to extend with."
        ));
    }

    // Combine addresses from file and command line
    let mut all_addresses = Vec::new();

    if let Some(addresses) = addresses {
        all_addresses.extend(addresses);
    }
    if let Some(file) = file {
        let file = File::open(file)?;
        let reader = BufReader::new(file);
        let addresses: Vec<String> = serde_json::from_reader(reader)?;
        all_addresses.extend(addresses);
    }

    let pubkeys = all_addresses
        .iter()
        .map(|address| Pubkey::from_str(address))
        .collect::<Result<Vec<Pubkey>, _>>()?;

    let ix = extend_lookup_table(
        lut_pubkey,
        authority_pubkey,
        Some(authority_pubkey),
        pubkeys,
    );

    let signature = send_transaction(&config, &[ix])?;

    println!(
        "Added addresses to LUT: {} with signature: {}",
        style(lut_address).bold().green(),
        style(signature).bold().green()
    );

    Ok(())
}

fn close_lut(lut_address: String) -> Result<()> {
    let config = setup::CliConfig::new()?;
    let authority_pubkey = config.keypair.pubkey();

    let lut_pubkey = Pubkey::from_str(&lut_address)?;

    let ix = close_lookup_table(lut_pubkey, authority_pubkey, authority_pubkey);

    let signature = send_transaction(&config, &[ix])?;

    println!(
        "Closed LUT: {} with signature: {}",
        style(lut_address).bold().green(),
        style(signature).bold().green()
    );

    Ok(())
}

fn deactivate_lut(lut_address: String) -> Result<()> {
    let config = setup::CliConfig::new()?;
    let authority_pubkey = config.keypair.pubkey();

    let lut_pubkey = Pubkey::from_str(&lut_address)?;

    let ix = deactivate_lookup_table(lut_pubkey, authority_pubkey);

    let signature = send_transaction(&config, &[ix])?;

    println!(
        "Deactivate LUT: {} with signature: {}",
        style(lut_address).bold().green(),
        style(signature).bold().green()
    );

    Ok(())
}

fn decode_lut(lut_address: String) -> Result<()> {
    let config = setup::CliConfig::new()?;

    let lut_pubkey = Pubkey::from_str(&lut_address)?;

    let account_data = config.client.get_account_data(&lut_pubkey)?;

    let addresses: Vec<Pubkey> = account_data[LOOKUP_TABLE_META_SIZE..]
        .chunks(32)
        .map(|chunk| {
            let mut array = [0u8; 32];
            array.copy_from_slice(chunk);
            Pubkey::new_from_array(array)
        })
        .collect();

    for address in addresses {
        println!("{}", address);
    }

    Ok(())
}
