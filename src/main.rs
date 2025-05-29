mod backend;
mod config;
mod connection;
mod protocol;
mod signer;
mod types;
mod versions;

use backend::{NativeSigner, SigningBackend};
use config::{Config, ProtocolVersionConfig};
use connection::open_secret_connection;
use log::{error, info};
use nebula::SignerError;
use protocol::{Request, Response};
use signer::Signer;
use std::io::{Read, Write};
use std::thread::sleep;
use std::time::Duration;
use std::{cmp::Ordering, net::TcpStream};
use tendermint_p2p::secret_connection::SecretConnection;
use types::{Proposal, SignedMsgType, Vote};
use versions::{ProtocolVersion, VersionV0_34, VersionV0_37, VersionV0_38, VersionV1_0};

fn main() -> Result<(), SignerError> {
    let config_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "config.toml".to_string());
    let config = Config::from_file(&config_path)?;

    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info)
        .init();

    info!("Loading config from: {}", config_path);
    info!("Chain ID: {}", config.chain_id);
    info!("Protocol version: {:?}", config.version);
    info!(
        "Connecting to {}:{}",
        config.connection.host, config.connection.port
    );

    let backend = NativeSigner::from_key_file(&config.private_key_path)?;

    let identity_key = ed25519_consensus::SigningKey::try_from(
        &std::fs::read(&config.connection.identity_key_path)?[..32],
    )?;

    let conn = open_secret_connection(
        &config.connection.host,
        config.connection.port,
        identity_key,
        config.connection.to_tendermint_version(),
    )?;

    info!("Starting request loop for chain: {}", config.chain_id);

    match config.version {
        ProtocolVersionConfig::V0_34 => {
            let mut signer = Signer::<NativeSigner, VersionV0_34, SecretConnection<TcpStream>>::new(
                backend,
                conn,
                config.chain_id,
            );
            run_signer_loop(&mut signer)?;
        }
        ProtocolVersionConfig::V0_37 => {
            let mut signer = Signer::<NativeSigner, VersionV0_37, SecretConnection<TcpStream>>::new(
                backend,
                conn,
                config.chain_id,
            );
            run_signer_loop(&mut signer)?;
        }
        ProtocolVersionConfig::V0_38 => {
            let mut signer = Signer::<NativeSigner, VersionV0_38, SecretConnection<TcpStream>>::new(
                backend,
                conn,
                config.chain_id,
            );
            run_signer_loop(&mut signer)?;
        }
        ProtocolVersionConfig::V1_0 => {
            let mut signer = Signer::<NativeSigner, VersionV1_0, SecretConnection<TcpStream>>::new(
                backend,
                conn,
                config.chain_id,
            );
            run_signer_loop(&mut signer)?;
        }
    }

    Ok(())
}

fn run_signer_loop<
    T: SigningBackend,
    V: versions::ProtocolVersion,
    C: std::io::Read + std::io::Write,
>(
    signer: &mut Signer<T, V, C>,
) -> Result<(), SignerError> {
    loop {
        match handle_single_request(signer) {
            Ok(()) => {}
            Err(e) => {
                error!("Error handling request: {}. Continuing...", e);
                sleep(Duration::from_millis(100));
            }
        }
    }
}

pub fn handle_single_request<T: SigningBackend, V: ProtocolVersion, C: Read + Write>(
    signer: &mut Signer<T, V, C>,
) -> Result<(), SignerError> {
    let req = signer.read_request()?;
    info!("Received request: {:?}", req);
    // TODO: pull real state from Raft or whatever
    let state = ConsensusData {
        height: 9999999,
        round: 99999,
        step: SignedMsgType::Unknown,
    };

    let response = match req {
        Request::SignProposal(proposal) => {
            if !should_sign_proposal(&state, &proposal) {
                Response::SignedProposal(V::create_signed_proposal_response(
                    None,
                    Vec::new(),
                    Some("would double-sign proposal at same height/round".into()),
                ))
            } else {
                signer.process_request(Request::SignProposal(proposal))?
            }
        }

        Request::SignVote(vote) => {
            if !should_sign_vote(&state, &vote) {
                Response::SignedVote(V::create_signed_vote_response(
                    None,
                    Vec::new(),
                    Some("would double-sign vote at same height/round".into()),
                ))
            } else {
                signer.process_request(Request::SignVote(vote))?
            }
        }

        other => signer.process_request(other)?,
    };

    signer.send_response(response)?;
    Ok(())
}

pub struct ConsensusData {
    height: i64,
    round: i64,
    step: SignedMsgType,
}

/*
A signer should only sign a proposal p if any of the following lines are true:

    p.Height > s.Height (1)
    p.Height == s.Height && p.Round > s.Round (2)

In other words, a proposal should only be signed if it’s at a higher height, or a higher round for the same height. Once a proposal or vote has been signed for a given height and round, a proposal should never be signed for the same height and round.
*/
fn should_sign_proposal(state: &ConsensusData, proposal: &Proposal) -> bool {
    // NOTE: possibly redundant? a safecheck if this is actually a proposal
    let msg_ty = SignedMsgType::from(proposal.msg_type);
    if msg_ty != SignedMsgType::Proposal {
        return false;
    }

    match (
        proposal.height.cmp(&state.height),
        proposal.round.cmp(&state.round),
    ) {
        // (1)
        (Ordering::Greater, _) => true,

        // (2)
        (Ordering::Equal, Ordering::Greater) => true,

        _ => false,
    }
}

/*
A signer should only sign a vote v if any of the following lines are true:

    v.Height > s.Height (1)
    v.Height == s.Height && v.Round > s.Round (2)
    v.Height == s.Height && v.Round == s.Round && v.Step == 0x1 && s.Step == 0x20 (3)
    v.Height == s.Height && v.Round == s.Round && v.Step == 0x2 && s.Step != 0x2 (4)

In other words, a vote should only be signed if it’s:
  - at a higher height
  - at a higher round for the same height
  - a prevote for the same height and round where we haven’t signed a prevote or precommit (but have signed a proposal)
  - a precommit for the same height and round where we haven’t signed a precommit (but have signed a proposal and/or a prevote)
*/
fn should_sign_vote(state: &ConsensusData, vote: &Vote) -> bool {
    let vote_step = SignedMsgType::from(vote.step);
    match (
        vote.height.cmp(&state.height),
        vote.round.cmp(&state.round),
        vote_step,
        state.step,
    ) {
        // (1)
        (Ordering::Greater, _, _, _) => true,

        // (2)
        (Ordering::Equal, Ordering::Greater, _, _) => true,

        // (3)
        (Ordering::Equal, Ordering::Equal, SignedMsgType::Prevote, SignedMsgType::Proposal) => true,

        // (4)
        (Ordering::Equal, Ordering::Equal, SignedMsgType::Precommit, stp)
            if stp != SignedMsgType::Precommit =>
        {
            true
        }

        // everything else: don't sign
        _ => false,
    }
}
