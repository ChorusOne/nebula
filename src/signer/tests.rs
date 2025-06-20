use super::mock_connection::MockCometBFTConnection;
use crate::backend::Ed25519Signer;
use crate::proto::v0_38;
use crate::protocol::Request;
use crate::signer::Signer;
use crate::types::SignedMsgType;
use crate::versions::VersionV0_38;
use prost::Message;
use std::time::Duration;

#[test]
fn signer_with_mock_connection() {
    let (mock_conn, handle) = MockCometBFTConnection::new();

    let backend = Ed25519Signer::from_key_file("./keys/privkey").unwrap();

    let mut signer = Signer::<_, VersionV0_38, _>::new(backend, mock_conn, "test-chain".into());

    let proposal_req = v0_38::privval::SignProposalRequest {
        proposal: Some(v0_38::types::Proposal {
            r#type: SignedMsgType::Proposal as i32,
            height: 1,
            round: 1,
            ..Default::default()
        }),
        chain_id: "test-chain".to_string(),
    };
    let msg = v0_38::privval::Message {
        sum: Some(v0_38::privval::message::Sum::SignProposalRequest(
            proposal_req,
        )),
    };
    let mut req_bytes = Vec::new();
    msg.encode_length_delimited(&mut req_bytes).unwrap();

    handle.request_sender.send(req_bytes).unwrap();

    let request = signer.read_request().unwrap();
    assert!(matches!(request, Request::SignProposal(_)));

    let response = signer.process_request(request).unwrap();

    signer.send_response(response).unwrap();

    let response_bytes = handle
        .response_receiver
        .recv_timeout(Duration::from_secs(1))
        .unwrap();
    let response_msg =
        v0_38::privval::Message::decode_length_delimited(response_bytes.as_slice()).unwrap();

    match response_msg.sum {
        Some(v0_38::privval::message::Sum::SignedProposalResponse(res)) => {
            assert!(res.error.is_none());
            let signed_proposal = res.proposal.unwrap();
            assert_eq!(signed_proposal.height, 1);
            assert_eq!(signed_proposal.round, 1);
            assert!(
                !signed_proposal.signature.is_empty(),
                "Signature should not be empty"
            );
            println!("Got signature: {}", hex::encode(&signed_proposal.signature));
        }
        _ => panic!("Expected a SignedProposalResponse"),
    }
}
