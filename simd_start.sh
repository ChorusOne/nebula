#!/bin/bash
set -e
set -x

CHAIN_ID="testing"
IP_ADDRESS="192.168.0.5"
HOME_BASE="/cow/pyoxa/.simapp-"
KEY_NAME_1="validator1"
KEY_NAME_2="validator2"
KEYRING="test"
STAKE_1="500000000stake"
STAKE_2="500000000stake"
SIMD_PATH="/var/home/pyoxa/git/cosmos-sdk/build/simd"

P2P_PORTS=(16556 26556 36556)
RPC_PORTS=(16557 26557 36557)
LCD_PORTS=(16317 26317 36317)
REMOTE_SIGNER_PORTS=(26558 36558)

tmux kill-session -t simapp || true
rm -rf ${HOME_BASE}1 ${HOME_BASE}2 ${HOME_BASE}3

$SIMD_PATH init "validator-1" --chain-id $CHAIN_ID --home ${HOME_BASE}1
$SIMD_PATH config set client chain-id testing --home ${HOME_BASE}1
$SIMD_PATH config set client keyring-backend test --home ${HOME_BASE}1

$SIMD_PATH keys add $KEY_NAME_1 --keyring-backend $KEYRING --home ${HOME_BASE}1
ADDRESS_1=$($SIMD_PATH keys show $KEY_NAME_1 -a --keyring-backend $KEYRING --home ${HOME_BASE}1)

$SIMD_PATH genesis add-genesis-account $ADDRESS_1 $STAKE_1 --home ${HOME_BASE}1

$SIMD_PATH keys add $KEY_NAME_2 --keyring-backend $KEYRING --home ${HOME_BASE}1
ADDRESS_2=$($SIMD_PATH keys show $KEY_NAME_2 -a --keyring-backend $KEYRING --home ${HOME_BASE}1)

$SIMD_PATH genesis add-genesis-account $ADDRESS_2 $STAKE_2 --home ${HOME_BASE}1

for i in 2 3; do
    $SIMD_PATH init "validator-$i" --chain-id $CHAIN_ID --home ${HOME_BASE}${i}
    $SIMD_PATH config set client chain-id testing --home ${HOME_BASE}${i}
    $SIMD_PATH config set client keyring-backend test --home ${HOME_BASE}${i}

    mkdir -p ${HOME_BASE}${i}/config
    cp -r ${HOME_BASE}1/keyring-test ${HOME_BASE}${i}/
done
cp ${HOME_BASE}1/config/genesis.json ${HOME_BASE}2/config/genesis.json
cp ${HOME_BASE}1/config/genesis.json ${HOME_BASE}3/config/genesis.json

PRIV_VALIDATOR_JSON='{
  "address": "08E80C9EB2573CB9626518FE182036CE3001FF36",
  "pub_key": {
    "type": "tendermint/PubKeyEd25519",
    "value": "vw0S42KWTmJXxziv8Rpn0ol7fb4ydW7B/h1yuMaUTyo="
  },
  "priv_key": {
    "type": "tendermint/PrivKeyEd25519",
    "value": "S9l7e7A2yJsEkV1dHG3+yrTjqE2eRHmeqAiJdQsFMQS/DRLjYpZOYlfHOK/xGmfSiXt9vjJ1bsH+HXK4xpRPKg=="
  }
}'

echo "$PRIV_VALIDATOR_JSON" > ${HOME_BASE}2/config/priv_validator_key.json
echo "$PRIV_VALIDATOR_JSON" > ${HOME_BASE}3/config/priv_validator_key.json

chmod 600 ${HOME_BASE}2/config/priv_validator_key.json
chmod 600 ${HOME_BASE}3/config/priv_validator_key.json



rm -f ${HOME_BASE}1/config/gentx/*.json

$SIMD_PATH genesis gentx $KEY_NAME_1 $STAKE_1 --chain-id $CHAIN_ID --home ${HOME_BASE}1 --keyring-backend $KEYRING
$SIMD_PATH genesis gentx $KEY_NAME_2 $STAKE_2 --chain-id $CHAIN_ID --home ${HOME_BASE}2 --keyring-backend $KEYRING

cp ${HOME_BASE}2/config/gentx/* ${HOME_BASE}1/config/gentx/
$SIMD_PATH genesis collect-gentxs --home ${HOME_BASE}1
$SIMD_PATH genesis validate-genesis --home ${HOME_BASE}1

cp ${HOME_BASE}1/config/genesis.json ${HOME_BASE}2/config/genesis.json
cp ${HOME_BASE}1/config/genesis.json ${HOME_BASE}3/config/genesis.json

for i in 1 2 3; do
    P2P_PORT=${P2P_PORTS[$((i-1))]}
    RPC_PORT=${RPC_PORTS[$((i-1))]}
    LCD_PORT=${LCD_PORTS[$((i-1))]}

    CONFIG_FILE="${HOME_BASE}${i}/config/config.toml"

    sed -i.bak "s|^laddr = \"tcp://0.0.0.0:26656\"|laddr = \"tcp://${IP_ADDRESS}:${P2P_PORT}\"|" $CONFIG_FILE
    sed -i.bak "s|^laddr = \"tcp://127.0.0.1:26657\"|laddr = \"tcp://${IP_ADDRESS}:${RPC_PORT}\"|" $CONFIG_FILE
    sed -i.bak "s|^laddr = \"tcp://0.0.0.0:1317\"|laddr = \"tcp://${IP_ADDRESS}:${LCD_PORT}\"|" $CONFIG_FILE
    sed -i.bak "s|^external_address = \"\"|external_address = \"${IP_ADDRESS}:${P2P_PORT}\"|" $CONFIG_FILE

    if [ $i -ne 1 ]; then
        SIGNER_PORT=${REMOTE_SIGNER_PORTS[$((i-2))]}
        sed -i.bak "s|^priv_validator_laddr =.*|priv_validator_laddr = \"tcp://0.0.0.0:${SIGNER_PORT}\"|" $CONFIG_FILE
    fi
done

PEER1=$($SIMD_PATH tendermint show-node-id --home ${HOME_BASE}1)@$IP_ADDRESS:16556
PEER2=$($SIMD_PATH tendermint show-node-id --home ${HOME_BASE}2)@$IP_ADDRESS:26556
PEER3=$($SIMD_PATH tendermint show-node-id --home ${HOME_BASE}3)@$IP_ADDRESS:36556

sed -i.bak "s|^persistent_peers =.*|persistent_peers = \"$PEER2,$PEER3\"|" ${HOME_BASE}1/config/config.toml
sed -i.bak "s|^persistent_peers =.*|persistent_peers = \"$PEER1,$PEER3\"|" ${HOME_BASE}2/config/config.toml
sed -i.bak "s|^persistent_peers =.*|persistent_peers = \"$PEER1,$PEER2\"|" ${HOME_BASE}3/config/config.toml

for i in 1 2 3; do
    CONFIG_FILE="${HOME_BASE}${i}/config/config.toml"
    sed -i.bak 's/^allow_duplicate_ip = false/allow_duplicate_ip = true/' $CONFIG_FILE
done

tmux new-session -d -s "simapp" -n "simapp-1" "$SIMD_PATH start --home ${HOME_BASE}1"
tmux new-window -t "simapp:1" -n "simapp-2" "$SIMD_PATH start --home ${HOME_BASE}2"
tmux new-window -t "simapp:2" -n "simapp-3" "$SIMD_PATH start --home ${HOME_BASE}3"
