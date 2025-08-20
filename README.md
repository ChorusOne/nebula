# Nebula - the CometBFT Remote signer, written in Rust

# NOTE: THIS IS AN ALPHA VERSION OF THE SIGNER!

Nebula is a CometBFT remote signer. It uses Raft to create a cluster of signer nodes that collectively maintain the signature high water mark.

## Principles, core assumptions

The core principle of Nebula is that the decision to sign a block is treated as a state transition in a distributed state machine.
A signature is only produced and transmitted *after* the state transition has been successfully committed to a quorum of nodes in the Raft cluster.
There is only ONE signer (Raft leader) at a time that is capable of connecting to CometBFT nodes.
Nebula tries to err on the side of signing less, than actually signing more, so in turbulent leadership changes, uptime is expected to suffer slightly.

### Sequence of a Signing Request

Consider a "happy" case, when leader of the Nebula cluster receives a signing request, e.g a proposal at height 100, at round 1. The flow is as follows:

1.  A mutex is acquired to ensure only one signing request is processed at a time. (`src/handler.rs`)
2.  The node verifies it is still the Raft leader. If not, it bails early.
3. The request's Height/Round/Step (HRS) is checked against the last known committed state. If signing would violate CometBFT's double-signing rules, the request is rejected. This logic is in `src/safeguards.rs`.
4. Leader proposes the new HRS state (`{h: 100, r: 1, step: Proposal}`) to the Raft cluster. The handler thread then **blocks** and waits for confirmation that this entry has been committed by a majority of the cluster.
    -   This is implemented in `SignerRaftNode::replicate_state` (`src/cluster/mod.rs`), which uses an `mpsc::channel` to wait for a callback.
    -   The callback is only sent by the Raft machinery in `handle_committed_entries` after the entry has been written to the distributed log.
    -   If a quorum cannot be reached, this step will time out and return an error.
5.  Leader usees the configured signing backend to produce a signature.
6.  The signature is sent back to the CometBFT validator.
7.  Mutex acquired at the beginning is r``eleased.

## Testing Strategy and Limitations

Nebula is currently evaluated primarily through integration tests.

-   The test suite in `src/cluster/integration_tests.rs` creates an in-memory cluster of multiple Raft nodes.
-   The network connection to the CometBFT validator is mocked.
-   The tests simulate failures by shutting down nodes, transferring leadership, and sending duplicate or out-of-order requests. Some of the test cases include:
    -   `double_sign_prevention_after_leadership_change`: Directly validates the failure scenario described above.
    -   `no_replicate_acks`: Confirms that signing fails if a leader cannot form a quorum, which is the correct safe behavior.
    -   `new_leader_signing`: Ensures that a newly elected leader correctly loads state and can proceed with signing *new* blocks.

### Current Limitations

-   They **do not** test the physical network I/O layers. Bugs in the TCP stream handling or `secret_connection` layer would not be caught.
-   Fault injection is currently programmatic (shutting down threads) rather than simulating true network partitions.
-   Probably a lot more which I have not thought about yet

## Supported Backends and Features

-   Signing Backends:
    -   `native`: Key is stored in a local file.
    -   `vault_transit`: Uses HashiCorp Vault's Transit engine.
    -   `vault_signer_plugin`: Uses a custom Vault plugin.
-   Native key types: Ed25519, Secp256k1, Bls12381.

## Usage
