use candid::{decode_one, encode_one, CandidType, Deserialize, Principal};
use pocket_ic::{PocketIc, WasmResult};
use std::{fs::read, path, path::PathBuf};

/// Test setup.
struct TestSetup {
    /// The PocketIC instance.
    pic: PocketIc,
    /// The id of the canister providing the API.
    api_canister_id: Principal,
}
impl TestSetup {
    pub fn new() -> Self {
        let pic = PocketIc::new();
        let api_canister_id = pic.create_canister();
        let wasm_bytes = Self::wasm_bytes();
        pic.add_cycles(api_canister_id, 1_000_000_000_000);
        pic.install_canister(api_canister_id, wasm_bytes, vec![], None);
        Self {
            pic,
            api_canister_id,
        }
    }
    fn wasm_path() -> PathBuf {
        path::absolute("../../target/wasm32-unknown-unknown/release/example.wasm").unwrap()
    }
    /// Reads the backend Wasm bytes from the configured path.
    fn wasm_bytes() -> Vec<u8> {
        let path = Self::wasm_path();
        read(&path).expect(&format!("Could not find the backend wasm: {path:?}"))
    }
    /// Makes an update call to the canister.
    fn update<T>(&self, caller: Principal, method: &str, arg: impl CandidType) -> Result<T, String>
    where
        T: for<'a> Deserialize<'a> + CandidType,
    {
        self.pic
            .update_call(
                self.api_canister_id.clone(),
                caller,
                method,
                encode_one(arg).unwrap(),
            )
            .map_err(|e| {
                format!(
                    "Update call error calling method '{method}'. RejectionCode: {:?}, Error: {}",
                    e.code, e.description
                )
            })
            .and_then(|reply| match reply {
                WasmResult::Reply(reply) => decode_one(&reply)
                    .map_err(|e| format!("Decoding response from '{method}' failed: {e}")),
                WasmResult::Reject(error) => Err(error),
            })
    }
}

#[test]
fn test_counter_canister() {
    let setup = TestSetup::new();
    setup
        .update::<String>(Principal::anonymous(), "free", ())
        .expect("Failed to make free API call");
}
