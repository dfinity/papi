use crate::util::pic_canister::PicCanisterTrait;
use crate::util::test_environment::TestSetup;
use candid::Principal;
use ic_papi_api::PaymentType;
use ic_papi_wrapper::domain::types::Call0Args;

#[test]
fn bridge_call_fails_if_target_is_self() {
    let setup = TestSetup::default();
    let args = Call0Args {
        target: setup.wrapper.canister_id(),
        method: "any".to_string(),
        fee_amount: 0,
        payment: Some(PaymentType::AttachedCycles),
        cycles_to_forward: None,
    };

    let result: Result<Result<Vec<u8>, String>, String> =
        setup.wrapper.update(setup.user, "call0", args);
    let inner_result = result.expect("Failed to reach canister");
    let err = inner_result.expect_err("Should have returned an error");
    assert!(err.contains("Self-calls are not allowed through the bridge."));
}

#[test]
fn bridge_call_fails_if_insufficient_cycles() {
    let setup = TestSetup::default();
    let args = Call0Args {
        target: setup.target.canister_id(),
        method: "any_method".to_string(),
        fee_amount: 1000,
        payment: Some(PaymentType::AttachedCycles),
        cycles_to_forward: None,
    };

    let result: Result<Result<Vec<u8>, String>, String> =
        setup.wrapper.update(setup.user, "call0", args);
    let inner_result = result.expect("Failed to reach canister");
    let err = inner_result.expect_err("Should have failed due to insufficient cycles");
    // Since we didn't attach cycles, it should fail with Payment guard error: Insufficient funds...
    assert!(err.contains("Payment guard error"));
}
