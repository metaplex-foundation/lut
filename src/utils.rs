use anyhow::Result;
use solana_sdk::{instruction::Instruction, signature::Signature, signer::Signer};

use crate::setup::CliConfig;

pub(crate) fn send_transaction(config: &CliConfig, ixes: &[Instruction]) -> Result<Signature> {
    let tx = solana_sdk::transaction::Transaction::new_signed_with_payer(
        ixes,
        Some(&config.keypair.pubkey()),
        &[&config.keypair],
        config.recent_blockhash,
    );

    let signature = config
        .client
        .send_and_confirm_transaction_with_spinner(&tx)?;

    Ok(signature)
}
