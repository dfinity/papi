use crate::pic_tool::{PicCanister, PicCanisterTrait};
use candid::{decode_one, encode_one, CandidType, Deserialize, Principal};
use pocket_ic::{PocketIc, WasmResult};
use std::fs;
use std::sync::Arc;

pub struct AttachedCyclesTestSetup {
    /// The PocketIC instance.
    pic: Arc<PocketIc>,
    /// The canister providing the API.
    api_canister: PicCanister,
    /// The canister consuming the API.
    customer_canister: PicCanister,
}
impl Default for AttachedCyclesTestSetup {
    fn default() -> Self {
        let pic = Arc::new(PocketIc::new());
        let api_canister = PicCanister::new(pic.clone(), &PicCanister::cargo_wasm_path("example"));
        let customer_canister = PicCanister::new(
            pic.clone(),
            &PicCanister::cargo_wasm_path("example_backend"),
        );
        Self {
            pic,
            api_canister,
            customer_canister,
        }
    }
}

#[test]
fn test_setup_works() {
    let _setup = AttachedCyclesTestSetup::default();
}
