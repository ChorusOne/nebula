mod backend;
mod cluster;
mod config;
mod connection;
mod error;
mod handler;
mod leader;
mod proto;
mod protocol;
mod safeguards;
mod signer;
mod types;
mod versions;

use crate::error::SignerError;
use clap::{Parser as _, Subcommand};
use cluster::SignerRaftNode;
use config::{Config, ProtocolVersionConfig};
use log::{error, info, warn};
use std::str::FromStr;
use versions::{VersionV0_34, VersionV0_37, VersionV0_38, VersionV1_0};

#[derive(clap::Parser)]
#[command(name = "signer")]
#[command(about = "A distributed CometBFT remote signer")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Start {
        #[arg(short, long = "config")]
        config_path: String,
    },
    Init {
        #[arg(short, long)]
        output_path: String,

        #[arg(short, long)]
        backend: config::SigningMode,
    },
}

fn main() -> Result<(), SignerError> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Start { config_path } => {
            let config = Config::from_file(&config_path)?;
            env_logger::Builder::new()
                .filter_level(
                    log::LevelFilter::from_str(&config.log_level).unwrap_or(log::LevelFilter::Info),
                )
                .init();
            start_signer(config)
        }
        Commands::Init {
            output_path,
            backend,
        } => {
            let default_config = Config::default_config(backend);
            default_config.write_to_file(&output_path)?;
            println!("Generated default configuration at '{}'", output_path);
            Ok(())
        }
    }
}

fn start_signer(config: Config) -> Result<(), SignerError> {
    info!("Chain ID: {}", config.chain_id);
    info!("Protocol version: {:?}", config.version);
    info!("Node ID: {}", config.raft.node_id);

    let raft_node = SignerRaftNode::new(config.raft.clone());

    loop {
        leader::wait_for_leader(&raft_node);

        if raft_node.is_leader() {
            info!("This node ({}) is now the leader!", config.raft.node_id);

            let result = match config.version {
                ProtocolVersionConfig::V0_34 => {
                    leader::run_leader::<VersionV0_34>(&config, &raft_node)
                }
                ProtocolVersionConfig::V0_37 => {
                    leader::run_leader::<VersionV0_37>(&config, &raft_node)
                }
                ProtocolVersionConfig::V0_38 => {
                    leader::run_leader::<VersionV0_38>(&config, &raft_node)
                }
                ProtocolVersionConfig::V1_0 => {
                    leader::run_leader::<VersionV1_0>(&config, &raft_node)
                }
            };

            match result {
                Ok(()) => warn!("Leader loop exited normally"),
                Err(e) => error!("Leader loop error: {}", e),
            }
        } else {
            leader::wait_as_follower(&raft_node);
        }
    }
}
