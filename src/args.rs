use clap::{
    builder::ArgAction,
    {Parser, Subcommand},
};

#[derive(Parser)]
#[clap(author, version, about)]
pub struct Args {
    #[arg(short = 'T', long, global = true, default_value = "60")]
    timeout: u32,

    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Clone, Subcommand)]
pub enum Commands {
    Create,
    Extend {
        lut: String,

        /// Space separated list of addresses
        #[arg(short, long, action = ArgAction::Append)]
        addresses: Option<Vec<String>>,

        /// JSON file with list of addresses
        #[arg(short, long)]
        file: Option<String>,
    },
    Close {
        lut: String,
    },
    Deactivate {
        lut: String,
    },
    Decode {
        lut: String,
    },
}
