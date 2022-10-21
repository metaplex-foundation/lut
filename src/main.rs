use std::str::FromStr;

use anyhow::Result;
use clap::Parser;
use console::style;
use solana_address_lookup_table_program::instruction::{
    close_lookup_table, create_lookup_table, deactivate_lookup_table, extend_lookup_table,
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
        args::Commands::Extend { lut, addresses } => extend_lut(lut, addresses)?,
        args::Commands::Close { lut } => close_lut(lut)?,
        args::Commands::Deactivate { lut } => deactivate_lut(lut)?,
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

fn extend_lut(lut_address: String, addresses: Vec<String>) -> Result<()> {
    let config = setup::CliConfig::new()?;
    let authority_pubkey = config.keypair.pubkey();

    let lut_pubkey = Pubkey::from_str(&lut_address)?;
    let addresses: Vec<Pubkey> = addresses
        .iter()
        .map(|address| Pubkey::from_str(address))
        .collect::<Result<Vec<Pubkey>, _>>()?;

    let ix = extend_lookup_table(
        lut_pubkey,
        authority_pubkey,
        Some(authority_pubkey),
        addresses,
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
