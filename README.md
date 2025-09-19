# Nebula - the CometBFT Remote signer, written in Rust

# NOTE: THIS IS AN ALPHA VERSION OF THE SIGNER!

Nebula is a CometBFT remote signer. It uses Raft to create a cluster of signer nodes that collectively maintain the signature high water mark.

## Principles, core assumptions

The core principle of Nebula is that the decision to sign a block is treated as a state transition in a distributed state machine.

A signature is only produced and transmitted *after* the state transition has been successfully committed to a quorum of nodes in the Raft cluster.

There is only ONE signer (Raft leader) at a time that is capable of connecting to CometBFT nodes. (That is also enforced by the privval protocol)

Nebula tries to err on the side of signing less, than actually signing more, so in turbulent leadership changes, uptime is expected to suffer slightly.

Nebula connects to only one blockchain node, with only one consensus key. That means you will need one instance per identity on a network.

### Why I think it's correct

In order to NOT double-sign, privval protocol requires a signer to track last signed state (HRS).
Following this, a cluster of privval signers must always agree on the last signed HRS. If Nebula can achieve that, then it's safe.

Raft provides a single, append-only, majority-agreed log with leader completeness and log matching; new leaders contain all committed entries, because a candidate cannot win an election unless its log is up to date.
A log entry containing the HRS is considered committed once the leader that created the entry has replicated it on a majority of servers.
Combining this with RocksDB's synchronous writes, it means that the "last signed HRS" is only advanced and sent to the CometBFT node when it is durably stored in a quorum.
Leaders can crash, restart, or be replaced, but Raft guarantees that the new leader must include all committed entries in its log, so the recorded HRS cannot roll back.
Thus the system never loses track of the last signed HRS, and double-signing cannot occur.


#### Failure modes and the expected outcomes
TODO. Is it needed though?


### Sequence of a Signing Request

Consider a "happy" case, where leader of the Nebula cluster connected to a single CometBFT node receives a signing request, e.g a proposal at height 100, at round 1. The flow is as follows:

1. A mutex is acquired to ensure requests from only one node is processed at a time. (`src/handler.rs`)
2. The node verifies it is still the Raft leader. If not, it bails early.
3. The request's Height/Round/Step (HRS) is checked against the last known committed state. If signing would violate CometBFT's double-signing rules, the request is rejected. This logic is in `src/safeguards.rs`.
4. Leader proposes the new HRS state (`{h: 100, r: 1, step: Proposal}`) to the Raft cluster. The handler thread then **blocks** and waits for confirmation that this entry has been committed by a majority of the cluster.
    -   This is implemented in `SignerRaftNode::replicate_state` (`src/cluster/mod.rs`), which uses an `mpsc::channel` to wait for a callback.
    -   The callback is only sent by the Raft machinery in `handle_committed_entries` after the entry has been written to the distributed log.
    -   If a quorum cannot be reached, this step will time out and return an error.
5.  Leader usees the configured signing backend to produce a signature.
6.  The signature is sent back to the CometBFT validator.
7.  Mutex acquired at the beginning is released.

After which, CometBFT node propagates the signature.

Now, consider a very similar case, but the Nebula leader is connected to two CometBFT nodes, say nodes A and B. Nebula received a signing request from node A first.
For node A, the signing flow looks identical to the one described above. Node B will send the same request, and what happens is as follows:

1. Handler will wait on the mutex until Node A's request is served.
2. The node verifies it is still the Raft leader. If not, it bails early.
3. The request's Height/Round/Step (HRS) is checked against the last known committed state. Because this request is the same as one served just before, it will fail here. Nebula will log: `Prevented double signing vote`, and the CometBFT node will report an error:
```
failed signing vote err="signerEndpoint returned error #1: Would double-sign vote at same height/round" height=100 module=consensus round=1 vote={"block_id":{"hash":"41648E00251B1F6A94089BF7F4D942B640665325863F0D92E44D36AEBB604904","parts":{"hash":"DDCA7D5234BC6EB67F2E91E68CC22E434C3B2BA5D5D77CFAA44D9FC0D254AC5F","total":1}},"extension":null,"extension_signature":null,"height":"100","round":1,"signature":null,"timestamp":"2025-08-20T15:11:30.581382895Z","type":1,"validator_address":"0F38A435D89DF98B10BE57928BA79111D7440379","validator_index":23}
```

This concludes the signing request, and only one signature will be transmitted to a CometBFT node.


## Testing Strategy and Limitations

Nebula is currently evaluated primarily through integration tests.

-   The test suite in `src/cluster/integration_tests.rs` creates an in-memory cluster of multiple Raft nodes.
-   The network connection to the CometBFT validator is mocked.
-   The tests simulate failures by shutting down nodes, transferring leadership, and sending duplicate or out-of-order requests.

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

Run `init` with `--help` to check available backends to bootstrap with:
```
nebula init --help
Usage: nebula init --output-path <OUTPUT_PATH> --backend <BACKEND>

Options:
  -o, --output-path <OUTPUT_PATH>
  -b, --backend <BACKEND>          [possible values: vault-transit, vault-signer-plugin, native]
```


Generate a default config file at `test.toml`:
```
nebula init --output-path test.toml --backend native
```

Example output:

```
log_level = "info"
chain_id = "test-chain-v1"
version = "v1_0"
signing_mode = "native"

[[connections]]
host = "127.0.0.1"
port = 36558

[[connections]]
host = "127.0.0.1"
port = 26558

[raft]
node_id = 1
bind_addr = "127.0.0.1:8080"
data_path = "./raft_data"
initial_state_path = "./initial_state.json"

[[raft.peers]]
id = 1
addr = "127.0.0.1:7001"


[signing.native]
private_key_path = "./privkey"
key_type = "ed25519"
```

`connections` array is the list of CometBFT the Nebula leader will connect to.
`host` and `port` must be set in accordace with `priv_validator_laddr` in CometBFT's config.toml.

`raft` section defines settings for the Raft signer cluster.
Peer list must include EVERY member of the cluster, including the very node you're setting up.

For native signing, you can generate keys with `./target/release/nebula keys generate --key-type ed25519`.
Example output:
```
private key: UINHR9vYjWllyqYo+Jxc5fjYUBox3eRygj6dbUughIE=
{
  "address": "D79D2CD52901D6D42D7617B977447EEC1CA00887",
  "priv_key": {
    "type": "tendermint/PrivKeyEd25519",
    "value": "UINHR9vYjWllyqYo+Jxc5fjYUBox3eRygj6dbUughIEVDPizljxIzHVAz2b6YuTdmuLUTgn12On0wXXxyEkhHw=="
  },
  "pub_key": {
    "type": "tendermint/PubKeyEd25519",
    "value": "FQz4s5Y8SMx1QM9m+mLk3Zri1E4J9djp9MF18chJIR8="
  }
}
```

In this case, `UINHR9vYjWllyqYo+Jxc5fjYUBox3eRygj6dbUughIE=` must be put under ./privkey.
