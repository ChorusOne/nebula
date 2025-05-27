use crate::Signer;
use crate::SigningBackend;
use crate::signer::InsufficientData;
use crate::signer::{Request, Response};
use crate::versions::VersionV0_38;
use nebula::proto::v0_38;
use prost::Message;
use std::io::{Cursor, Read, Write};

struct Dummy;

impl SigningBackend for Dummy {
    fn sign(&self, _: &[u8]) -> Vec<u8> {
        vec![0xde, 0xad, 0xbe, 0xef]
    }
    fn public_key(&self) -> Vec<u8> {
        vec![1, 2, 3, 4]
    }
}

type TestSigner = Signer<Dummy, VersionV0_38, Cursor<Vec<u8>>>;

// TODO: maybe this can be pulled in from actual code
fn create_ping_message() -> Vec<u8> {
    let msg = v0_38::privval::Message {
        sum: Some(v0_38::privval::message::Sum::PingRequest(
            v0_38::privval::PingRequest {},
        )),
    };
    let mut buf = Vec::new();
    msg.encode_length_delimited(&mut buf).unwrap();
    buf
}

#[test]
fn ping_request() {
    let ping_data = create_ping_message();
    let mut s = TestSigner::new(Dummy, Cursor::new(ping_data), "test-chain".into());

    let req = s.read_request().unwrap();
    assert!(matches!(req, Request::PingRequest));

    // todo: maybe process_request should be split further?
    let resp = s.process_request(req).unwrap();
    assert!(matches!(resp, Response::Ping(_)));

    assert!(s.send_response(resp).is_ok());
}

#[test]
fn truncated_stream() {
    let s = TestSigner::new(Dummy, Cursor::new(vec![]), "test-chain".into());
    let ping_data = create_ping_message();

    let partial = &ping_data[..ping_data.len() - 1];
    let result = s.try_read_complete_message(partial);
    assert!(matches!(result, Err(InsufficientData::NeedMoreBytes)));
}

#[test]
fn normal_stream() {
    let s = TestSigner::new(Dummy, Cursor::new(vec![]), "test-chain".into());
    let ping_data = create_ping_message();

    let result = s.try_read_complete_message(&ping_data);
    assert!(result.is_ok());
    let (message, consumed) = result.unwrap();
    assert_eq!(message, ping_data);
    assert_eq!(consumed, ping_data.len());
}

#[test]
fn additional_data() {
    let s = TestSigner::new(Dummy, Cursor::new(vec![]), "test-chain".into());
    let ping1 = create_ping_message();
    let ping2 = create_ping_message();
    let combined = [ping1.clone(), ping2].concat();

    let result = s.try_read_complete_message(&combined);
    assert!(result.is_ok());
    let (message, consumed) = result.unwrap();
    assert_eq!(message, ping1);
    assert_eq!(consumed, ping1.len());
}

#[test]
fn empty_buffer() {
    let s = TestSigner::new(Dummy, Cursor::new(vec![]), "test-chain".into());
    let result = s.try_read_complete_message(&[]);
    assert!(matches!(result, Err(InsufficientData::NeedMoreBytes)));
}

#[test]
fn partial_bodt() {
    let s = TestSigner::new(Dummy, Cursor::new(vec![]), "test-chain".into());
    let ping_data = create_ping_message();

    let partial_len = if ping_data.len() > 2 { 2 } else { 1 };
    let result = s.try_read_complete_message(&ping_data[..partial_len]);
    assert!(matches!(result, Err(InsufficientData::NeedMoreBytes)));
}

#[test]
fn basic_test() {
    let ping_data = create_ping_message();
    let mut s = TestSigner::new(Dummy, Cursor::new(ping_data.clone()), "test-chain".into());

    let result = s.read_complete_message().unwrap();
    assert_eq!(result, ping_data);
}

#[test]
fn multiple_messages_in_buffer() {
    let ping1 = create_ping_message();
    let ping2 = create_ping_message();
    let combined = [ping1.clone(), ping2.clone()].concat();

    let mut s = TestSigner::new(Dummy, Cursor::new(combined), "test-chain".into());

    let msg1 = s.read_complete_message().unwrap();
    assert_eq!(msg1, ping1);

    let msg2 = s.read_complete_message().unwrap();
    assert_eq!(msg2, ping2);
}
