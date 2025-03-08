#!/usr/bin/env bash
set -e  # Exit on error

if dfx canister id vetkd_notes --network $NETWORK 2>&1 > /dev/null; then
  echo "vetkd_notes already exists"
else
  dfx canister create vetkd_notes --network $NETWORK --identity $IDENTITY
fi
if dfx canister id ic_siwe_provider --network $NETWORK 2>&1 > /dev/null; then
  echo "ic_siwe_provider already exists"
else
  dfx canister create ic_siwe_provider --network $NETWORK --identity $IDENTITY
fi
if dfx canister id vetkd_system_api --network $NETWORK 2>&1 > /dev/null; then
  echo "vetkd_system_api already exists"
else
  dfx canister create vetkd_system_api --network $NETWORK --identity $IDENTITY
fi
if dfx canister id vetkd_www --network $NETWORK 2>&1 > /dev/null; then
  echo "vetkd_www already exists"
else
  dfx canister create vetkd_www --network $NETWORK --identity $IDENTITY
fi
if dfx canister id vetkd_system_api --network $NETWORK 2>&1 > /dev/null; then
  echo "vetkd_system_api already exists"
else
  dfx canister create vetkd_system_api --network $NETWORK --identity $IDENTITY
fi

touch ./packages/vetkd-notes-canister/src/lib.rs
dfx build vetkd_notes --network $NETWORK
LOCAL_HASH=$(sha256sum .dfx/$NETWORK/canisters/vetkd_notes/vetkd_notes.wasm | awk '{ print "0x" $1 }')
REMOTE_HASH=$(dfx canister info vetkd_notes --network "ic" | grep 'Module hash' | awk '{ print $3 }')
if [ "$LOCAL_HASH" != "$REMOTE_HASH" ]; then
  dfx deploy vetkd_notes --network $NETWORK --identity $IDENTITY
else
  echo "vetkd_notes is up to date"
fi

dfx deploy ic_siwe_provider --network ${NETWORK} --identity $IDENTITY --argument "$(cat <<EOF
(
    record {
        domain = "$DOMAIN";
        uri = "$URI";
        salt = "$SALT";
        chain_id = opt $CHAIN_ID;
        scheme = opt "https";
        statement = opt "$STATEMENT";
        sign_in_expires_in = opt 300000000000;
        session_expires_in = opt 604800000000000;
        targets = opt vec {
            "$(dfx canister id ic_siwe_provider --network $NETWORK)";
            "$(dfx canister id vetkd_notes --network $NETWORK)";
        };
    }
)
EOF
)"

LOCAL_HASH=$(sha256sum .dfx/$NETWORK/canisters/vetkd_system_api/vetkd_system_api.wasm | awk '{ print "0x" $1 }')
REMOTE_HASH=$(dfx canister info vetkd_system_api --network "ic" | grep 'Module hash' | awk '{ print $3 }')
if [ "$LOCAL_HASH" != "$REMOTE_HASH" ]; then
  dfx deploy vetkd_system_api --network $NETWORK --identity $IDENTITY
else
  echo "vetkd_system_api is up to date"
fi
dfx generate vetkd_notes --network $NETWORK --identity $IDENTITY

LOCAL_HASH=$(sha256sum .dfx/$NETWORK/canisters/vetkd_system_api/vetkd_system_api.wasm | awk '{ print "0x" $1 }')
REMOTE_HASH=$(dfx canister info vetkd_system_api --network "ic" | grep 'Module hash' | awk '{ print $3 }')
touch ./packages/frontend/src/main.js
dfx build vetkd_www --network $NETWORK
if [ "$LOCAL_HASH" != "$REMOTE_HASH" ]; then
  dfx deploy vetkd_www --network $NETWORK --identity $IDENTITY
else
  echo "vetkd_www is up to date"
fi
