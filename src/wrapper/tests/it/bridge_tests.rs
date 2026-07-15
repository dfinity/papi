use crate::util::pic_canister::PicCanisterTrait;
use crate::util::test_environment::TestSetup;
use candid::{decode_one, encode_args, Principal};
use ic_papi_api::PaymentType;
use ic_papi_wrapper::domain::types::{Call0Args, FeeDenom, FeeSpec, MethodConfig, MethodKey};

#[test]
fn bridge_call_fails_if_target_is_self() {
    let setup = TestSetup::default();
    let args = Call0Args {
        target: setup.wrapper.canister_id(),
        method: "any".to_string(),
        payment: Some(PaymentType::AttachedCycles),
    };

    let result: Result<Result<Vec<u8>, String>, String> =
        setup.wrapper.update(setup.user, "call0", args);
    let inner_result = result.expect("Failed to reach canister");
    let err = inner_result.expect_err("Should have returned an error");
    assert!(err.contains("Self-calls are not allowed through the bridge."));
}

#[test]
fn bridge_call_fails_if_target_is_management_canister() {
    // The management canister is a forbidden target: proxying to it would run
    // with the bridge's own principal as caller. This is rejected before any
    // pricing lookup, so no configuration is needed for the test.
    let setup = TestSetup::default();
    let args = Call0Args {
        target: Principal::management_canister(),
        method: "update_settings".to_string(),
        payment: Some(PaymentType::AttachedCycles),
    };

    let result: Result<Result<Vec<u8>, String>, String> =
        setup.wrapper.update(setup.user, "call0", args);
    let inner_result = result.expect("Failed to reach canister");
    let err = inner_result.expect_err("Should have returned an error");
    assert!(err.contains("the management canister may not be reached through the bridge."));
}

#[test]
fn bridge_call_fails_if_method_not_configured() {
    // With operator-controlled pricing, a call to a `(target, method)` that the
    // operator has not registered must be rejected outright -- there is no
    // caller-supplied fee to fall back on.
    let setup = TestSetup::default();
    let args = Call0Args {
        target: setup.target.canister_id(),
        method: "unconfigured_method".to_string(),
        payment: Some(PaymentType::AttachedCycles),
    };

    let result: Result<Result<Vec<u8>, String>, String> =
        setup.wrapper.update(setup.user, "call0", args);
    let inner_result = result.expect("Failed to reach canister");
    let err = inner_result.expect_err("Should have failed: method not configured");
    assert!(
        err.contains("No price is configured"),
        "unexpected error: {err}"
    );
}

#[test]
fn set_method_config_requires_controller() {
    // Pricing is operator-only: a non-controller caller must not be able to
    // register or change a method's price (and thus its forwarded cycles).
    let setup = TestSetup::default();
    let key = MethodKey {
        target: setup.target.canister_id(),
        method: "any".to_string(),
    };
    let config = MethodConfig {
        fee: FeeSpec {
            amount: 0,
            denom: FeeDenom::Cycles,
        },
        supported: vec![],
        forward_cycles: None,
    };

    let bytes = setup
        .pic
        .update_call(
            setup.wrapper.canister_id(),
            setup.user, // not a controller of the wrapper
            "set_method_config",
            encode_args((key, config)).unwrap(),
        )
        .expect("Failed to reach canister");
    let res: Result<(), String> = decode_one(&bytes).unwrap();
    let err = res.expect_err("A non-controller must not be able to set config");
    assert!(err.contains("controller"), "unexpected error: {err}");
}
