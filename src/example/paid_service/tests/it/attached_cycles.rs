use candid::Principal;
use ic_papi_api::PaymentError;
use crate::util::pic_canister::{PicCanister, PicCanisterTrait};
use pocket_ic::PocketIc;
use std::sync::Arc;

pub struct AttachedCyclesTestSetup {
    /// The PocketIC instance.
    #[allow(dead_code)] // The Arc is used; this makes it accessible without having to refer to a specific canister.
    pic: Arc<PocketIc>,
    /// The canister providing the API.
    api_canister: PicCanister,
    /// The canister consuming the API.
    customer_canister: PicCanister,
}
impl Default for AttachedCyclesTestSetup {
    fn default() -> Self {
        let pic = Arc::new(PocketIc::new());
        let api_canister = PicCanister::new(
            pic.clone(),
            &PicCanister::cargo_wasm_path("example_paid_service"),
        );
        let customer_canister = PicCanister::new(
            pic.clone(),
            &PicCanister::cargo_wasm_path("example_app_backend"),
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

#[test]
fn inter_canister_call_succeeds_with_sufficient_cycles_only() {
    let setup = AttachedCyclesTestSetup::default();
    for cycles in 995u64..1005 {
        let args = (
            setup.api_canister.canister_id(),
            "cost_1000_cycles".to_string(),
            cycles,
        );
        let result: Result<Result<String, PaymentError>, String> = setup.customer_canister.update(
            Principal::anonymous(),
            "call_with_attached_cycles",
            args,
        );
        let result = result.expect("Failed to reach paid API");
        if cycles < 1000 {
            assert_eq!(
                result,
                Err(PaymentError::InsufficientFunds {
                    needed: 1000,
                    available: cycles
                }),
                "Should have failed with only {} cycles attached",
                cycles
            );
        } else {
            assert_eq!(
                result,
                Ok("Yes, you paid 1000 cycles!".to_string()),
                "Should have succeeded with {} cycles attached",
                cycles
            );
        }
    }
}