#!/bin/bash
set -euxo pipefail

# TODO: Run only if needed
dfx deploy

export POCKET_IC_SERVER_VERSION=5.0.0
export POCKET_IC_SERVER_PATH="target/pocket-ic"
export POCKET_IC_BIN="../../../${POCKET_IC_SERVER_PATH}"
export POCKET_IC_MUTE_SERVER=""
scripts/pic-install

# Run tests
echo "Running integration tests."
cargo test -p example_paid_service "${@}"
