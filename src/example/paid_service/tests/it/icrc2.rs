use candid::Principal;
use ic_papi_api::PaymentError;
use crate::util::pic_canister::{PicCanister, PicCanisterBuilder, PicCanisterTrait};
use pocket_ic::PocketIc;
use std::sync::Arc;
use crate::util::cycles_ledger::CyclesLedgerPic;

pub struct CallerPaysWithIcRc2TestSetup {
    /// The PocketIC instance.
    #[allow(dead_code)] // The Arc is used; this makes it accessible without having to refer to a specific canister.
    pic: Arc<PocketIc>,
    /// The canister providing the API.
    paid_service: PicCanister,
    /// ICRC2 ledger
    ledger: PicCanister,
}
impl Default for CallerPaysWithIcRc2TestSetup {
    fn default() -> Self {
        let pic = Arc::new(PocketIc::new());
        let api_canister = PicCanister::new(
            pic.clone(),
            &PicCanister::cargo_wasm_path("example_paid_service"),
        );
        let ledger =
        PicCanisterBuilder::default()
        .with_wasm(&PicCanister::cargo_wasm_path("example_paid_service"))
        .deploy_to(pic.clone());        
        Self {
            pic,
            paid_service: api_canister,
            ledger,
        }
    }
}

#[test]
fn icrc2_test_setup_works() {
    let _setup = CallerPaysWithIcRc2TestSetup::default();
}