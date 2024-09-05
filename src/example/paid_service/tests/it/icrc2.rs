use crate::util::cycles_ledger::{CyclesLedgerPic, InitArgs, LedgerArgs};
use crate::util::pic_canister::{PicCanister, PicCanisterBuilder, PicCanisterTrait};
use candid::{encode_one, Principal};
use ic_papi_api::PaymentError;
use pocket_ic::PocketIc;
use std::sync::Arc;

pub struct CallerPaysWithIcRc2TestSetup {
    /// The PocketIC instance.
    #[allow(dead_code)]
    // The Arc is used; this makes it accessible without having to refer to a specific canister.
    pic: Arc<PocketIc>,
    /// The canister providing the API.
    paid_service: PicCanister,
    /// ICRC2 ledger
    ledger: PicCanister,
    /// User's cycles wallet
}
impl Default for CallerPaysWithIcRc2TestSetup {
    fn default() -> Self {
        let pic = Arc::new(PocketIc::new());
        let api_canister = PicCanister::new(
            pic.clone(),
            &PicCanister::cargo_wasm_path("example_paid_service"),
        );
        let ledger = PicCanisterBuilder::default()
            .with_wasm(&PicCanister::dfx_wasm_path("cycles_ledger"))
            .with_arg(
                encode_one(LedgerArgs::Init(InitArgs {
                    index_id: None,
                    max_blocks_per_request: 999,
                }))
                .expect("Failed to encode ledger init arg"),
            )
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

#[test]
fn icrc2_payment_works() {
    let setup = CallerPaysWithIcRc2TestSetup::default();

}