# Nebula - the CometBFT Remote signer, written in Rust


## Supported backends
- Native signing
- Vault transit module


## How to prevent double signing


```go
/*
A signer should only sign a proposal p if any of the following lines are true:

	p.Height > s.Height
	p.Height == s.Height && p.Round > s.Round

In other words, a proposal should only be signed if it’s at a higher height, or a higher round for the same height. Once a proposal or vote has been signed for a given height and round, a proposal should never be signed for the same height and round.
*/
func shouldSignProposal(cd data.ConsensusData, propHeight int64, propRound int32) bool {
	return (propHeight > cd.Height) || (propHeight == cd.Height && propRound > int32(cd.Round))
}

/*
A signer should only sign a vote v if any of the following lines are true:

	v.Height > s.Height
	v.Height == s.Height && v.Round > s.Round
	v.Height == s.Height && v.Round == s.Round && v.Step == 0x1 && s.Step == 0x20
	v.Height == s.Height && v.Round == s.Round && v.Step == 0x2 && s.Step != 0x2

In other words, a vote should only be signed if it’s:
  - at a higher height
  - at a higher round for the same height
  - a prevote for the same height and round where we haven’t signed a prevote or precommit (but have signed a proposal)
  - a precommit for the same height and round where we haven’t signed a precommit (but have signed a proposal and/or a prevote)
*/
func shouldSignVote(cd data.ConsensusData, voteHeight int64, voteRound int32, voteStep int8) bool {
	if voteHeight > cd.Height {
		return true
	}

	if voteHeight == cd.Height && voteRound > int32(cd.Round) {
		return true
	}

	if voteHeight == cd.Height &&
		voteRound == int32(cd.Round) &&
		voteStep == data.StepPrevote &&
		cd.Step == data.StepPropose {
		return true
	}

	if voteHeight == cd.Height &&
		voteRound == int32(cd.Round) &&
		voteStep == data.StepPrecommit &&
		cd.Step != data.StepPrecommit {
		return true
	}
	return false
}

func shouldSignVoteExtension(chainID string, signBz, extSignBz []byte) (bool, error) {
	var vote cmtypes.CanonicalVote
	if err := protoio.UnmarshalDelimited(signBz, &vote); err != nil {
		return false, nil
	}

	if vote.Type == cmtypes.PrecommitType && vote.BlockID != nil && len(extSignBz) > 0 {
		var ext cmtypes.CanonicalVoteExtension
		if err := protoio.UnmarshalDelimited(extSignBz, &ext); err != nil {
			return false, fmt.Errorf("failed to unmarshal vote extension: %w", err)
		}

		switch {
		case ext.ChainId != chainID:
			return false, fmt.Errorf("extension chain ID %s does not match expected %s", ext.ChainId, chainID)
		case ext.Height != vote.Height:
			return false, fmt.Errorf("extension height %d does not match vote height %d", ext.Height, vote.Height)
		case ext.Round != vote.Round:
			return false, fmt.Errorf("extension round %d does not match vote round %d", ext.Round, vote.Round)
		}

		return true, nil
	}

	return false, nil
}

```

if you are a signer:
1. load the config
2. validate everything, prepare to start
3. wait until you are the leader
4. once leader, start the polling loop
5. serve requests in loop, do not double sign as per rules above
6. if you lose leadership, go back to 3

Source for above: https://docs.cometbft.com/v1.0/spec/consensus/signing

https://arxiv.org/pdf/1807.04938

### Code generation from proto files

Install buf:
```
GOBIN=/usr/local/bin go install github.com/bufbuild/buf/cmd/buf@v1.54.0
```


Create an account at https://buf.build

Get a token, for example here: https://buf.build/cometbft/cometbft/sdks/main:community/neoeinstein-prost

Login so you won't get rate-limited:
```
cargo login --registry buf "Bearer {token}"
```

Generate files (you shouldn't need to do that):
```
buf generate --template buf.gen.yaml
```


   89  sudo apt update && sudo apt install vault
   94  vault secrets enable transit
   98  vault transit import transit/keys/validator-key @privkey.pk8.b64 type=ed25519 exportable=true
   99  vault transit import transit/keys/validator-key @b64_privkey type=ed25519 exportable=true
  100  vault read -format=json transit/keys/validator-key   | jq -r '.data.keys["1"].public_key
  103  vault read -format=json transit/keys/validator-key   | jq -r '.data.keys["1"].public_key
  115  vault server -dev -dev-root-token-id="root" -dev-listen-address="127.0.0.1:8200"
  125  vault read -format=json transit/keys/validator-key   | jq -r '.data.keys["1"].public_key
  129  vault read -format=json transit/keys/validator-key   | jq -r '.data.keys["1"].public_key
  131  vault secrets enable transit
  132  vault transit import transit/keys/validator-key @b64_privkey type=ed25519 exportable=true
  165  vault transit import transit/keys/validator-key-injective @b64_injective_privkey type=ed25519 exportable=true
  168  vault transit import transit/keys/validator-key-injective @b64_injective_privkey type=ed25519 exportable=true
  173  vault read -format=json transit/keys/validator-key-injective   | jq -r '.data.keys["1"].public_key
  183  vault read -format=json transit/keys/validator-key   | jq -r '.data.keys["1"].public_key
  189  vault read -format=json transit/keys/validator-key   | jq -r '.data.keys["1"].public_key
  191  vault read -format=json transit/keys/validator-key   | jq -r '.data.keys["1"]
  193  vault read -format=json transit/keys/validator-key   | jq -r '.data.keys["1"]'
  194  vault read -format=json transit/keys/validator-key   | jq -r '.data'
  207  history | grep vault
root@ha-signer03:~#
