use anyhow::{anyhow, Result};
use dirs::{config_dir, home_dir};
use serde::{Deserialize, Serialize};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    clock::Slot,
    commitment_config::CommitmentConfig,
    hash::Hash,
    signature::{read_keypair_file, Keypair},
};
use std::fs::File;

#[derive(Debug, Deserialize, Serialize)]
struct SolanaConfig {
    pub json_rpc_url: String,
    pub keypair_path: String,
    pub commitment: String,
}

pub(crate) struct CliConfig {
    pub client: RpcClient,
    pub keypair: Keypair,
    pub recent_blockhash: Hash,
    pub recent_slot: Slot,
}

impl CliConfig {
    pub fn new() -> Result<Self> {
        let config = parse_solana_config().expect("Couldn't find solana config");
        let keypair = read_keypair_file(&config.keypair_path)
            .map_err(|_| anyhow!("Couldn't read keypair file"))?;

        let client = solana_client::rpc_client::RpcClient::new_with_commitment(
            config.json_rpc_url,
            CommitmentConfig::confirmed(),
        );
        let recent_blockhash = client.get_latest_blockhash()?;
        let recent_slot = client.get_slot()?;

        Ok(Self {
            client,
            keypair,
            recent_blockhash,
            recent_slot,
        })
    }

    #[allow(unused)]
    pub fn update_blocks(&mut self) -> Result<()> {
        self.recent_blockhash = self.client.get_latest_blockhash()?;
        self.recent_slot = self.client.get_slot()?;

        Ok(())
    }
}

fn parse_solana_config() -> Option<SolanaConfig> {
    let config_path = config_dir().expect("Couldn't find config dir");

    let mut solana_config_path = config_path.join("solana").join("cli").join("config.yml");

    if !solana_config_path.exists() {
        solana_config_path = home_dir()
            .expect("Couldn't find home directory")
            .join(".config")
            .join("solana")
            .join("cli")
            .join("config.yml");
    }

    let config_file = File::open(solana_config_path).expect("Couldn't open config file");

    serde_yaml::from_reader(&config_file).ok()
}
