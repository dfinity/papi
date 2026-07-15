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
fn bridge_call_rejects_forward_exceeding_fee() {
    // Regression test for the cycle-drain: a caller must not be able to forward
    // more cycles than the fee it pays. Here the classic attack -- a zero fee
    // with a large forward amount -- must be rejected before any forwarding.
    let setup = TestSetup::default();
    let args = Call0Args {
        target: setup.target.canister_id(),
        method: "any_method".to_string(),
        fee_amount: 0,
        payment: Some(PaymentType::AttachedCycles),
        cycles_to_forward: Some(1_000_000_000_000),
    };

    let result: Result<Result<Vec<u8>, String>, String> =
        setup.wrapper.update(setup.user, "call0", args);
    let inner_result = result.expect("Failed to reach canister");
    let err = inner_result.expect_err("Should have rejected forward exceeding fee");
    assert!(
        err.contains("exceed the fee charged"),
        "unexpected error: {err}"
    );
}

#[test]
fn bridge_call_rejects_forward_with_token_payment() {
    // Token payments credit a token account, not the wrapper's cycle balance, so
    // forwarding cycles against them would drain the wrapper. Even with a fee
    // that nominally covers the forward, a token payment type must be rejected.
    let setup = TestSetup::default();
    let args = Call0Args {
        target: setup.target.canister_id(),
        method: "any_method".to_string(),
        fee_amount: 1_000_000_000_000,
        payment: Some(PaymentType::CallerPaysIcrc2Tokens(
            ic_papi_api::caller::CallerPaysIcrc2Tokens {
                ledger: Principal::anonymous(),
            },
        )),
        cycles_to_forward: Some(1_000),
    };

    let result: Result<Result<Vec<u8>, String>, String> =
        setup.wrapper.update(setup.user, "call0", args);
    let inner_result = result.expect("Failed to reach canister");
    let err = inner_result.expect_err("Should have rejected forward with token payment");
    assert!(
        err.contains("requires a cycle-denominated payment type"),
        "unexpected error: {err}"
    );
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
