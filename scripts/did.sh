#!/usr/bin/env bash

did_file_location_from_cargo() {
  # Warning: Does no _- conversion.
  cargo metadata --format-version 1 | jq -r --arg v "$1" '.packages[] | select(.name==$v) | .manifest_path | sub("Cargo.toml";"\($v).did")'
}
cargo_manifest_path() {
  # Warning: Does no _- conversion.
  cargo metadata --format-version 1 | jq -r --arg v "$1" '.packages[] | select(.name==$v) | .manifest_path")'
}
did_file_location_from_dfx_json() {
  # Warning: Does no _- conversion.
  jq -r --arg v "$1" '.canisters[$v].candid' dfx.json
}

cargo_package_from_dfx_json() {
  jq -r --arg v "$1" '.canisters[$v].package' dfx.json
}

function generate_did() {
  local canister=$1
  local package
  package="$(cargo_package_from_dfx_json "$canister")"
  local wasm_name="${package//-/_}"

  echo "Deriving candid file from Rust for $package"
  local candid_file
  candid_file="$(did_file_location_from_dfx_json "$canister")"

  test -e "target/wasm32-unknown-unknown/release/$wasm_name.wasm" ||
    cargo build \
      --target wasm32-unknown-unknown \
      --release --package "$package"

  # cargo install candid-extractor
  candid-extractor "target/wasm32-unknown-unknown/release/$wasm_name.wasm" >"$candid_file"
  echo "Written: $candid_file"
}

CANISTERS=(example_app_backend example_paid_service wrapper)

for canister in "${CANISTERS[@]}"; do
  generate_did "$canister"
done
