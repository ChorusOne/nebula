vault server -dev -dev-root-token-id="root" -dev-listen-address="127.0.0.1:8200"

export VAULT_ADDR="http://127.0.0.1:8200"
export VAULT_TOKEN="root"

vault secrets enable transit

vault write transit/keys/validator-key \
  type=ed25519 \
  exportable=true

vault read -format=json transit/keys/validator-key \
  | jq -r '.data.keys["1"].public_key'
