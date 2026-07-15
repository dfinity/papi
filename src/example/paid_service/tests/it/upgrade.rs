//! Regression tests: the paid service must remain usable after a canister upgrade.
//!
//! The init args (which include the payment ledger) are held in a non-stable `thread_local`.
//! Without a `post_upgrade` hook to restore them, `cost_1b` traps with "No init args provided"
//! after any upgrade. See the `pre_upgrade`/`post_upgrade` hooks in `src/lib.rs`.
use crate::util::pic_canister::PicCanisterTrait;
use crate::util::test_environment::{PaidMethods, TestSetup, LEDGER_FEE};
use example_paid_service_api::InitArgs;
use ic_papi_api::caller::CallerPaysIcrc2Tokens;
use ic_papi_api::cycles::cycles_ledger_canister_id;
use ic_papi_api::{PaymentError, PaymentType};

/// Drives a `cost_1b` call and asserts it succeeds, i.e. the payment ledger config is available.
fn assert_cost_1b_succeeds(setup: &TestSetup) {
    let method = PaidMethods::Cost1b;
    setup.user_approves_payment_for_paid_service(method.cost() + LEDGER_FEE);
    let response: Result<String, PaymentError> = setup.call_paid_service(
        setup.user,
        method,
        PaymentType::CallerPaysIcrc2Tokens(CallerPaysIcrc2Tokens {
            ledger: cycles_ledger_canister_id(),
        }),
    );
    assert_eq!(
        response,
        Ok("Yes, you paid 1 billion cycles!".to_string()),
        "cost_1b should succeed after a canister upgrade",
    );
}

/// Verifies that `cost_1b` still works after an upgrade that relies on stable memory.
///
/// The upgrade happens before any call has lazily initialised the payment guard, so the guard is
/// first built (reading the init args) only after the upgrade. This is exactly the path that used
/// to trap when the init args were not persisted across upgrades.
#[test]
fn cost_1b_works_after_upgrade_restoring_from_stable_memory() {
    let setup = TestSetup::default();
    // Upgrade with no explicit args: the init args must be restored from stable memory.
    setup.upgrade_paid_service(None);
    assert_cost_1b_succeeds(&setup);
}

/// Verifies that init args supplied explicitly at upgrade time are used.
///
/// This is the path an operator takes when upgrading from a version that never persisted its args
/// (e.g. one without `pre_upgrade`): the ledger config is provided in the upgrade itself.
#[test]
fn cost_1b_works_after_upgrade_with_explicit_args() {
    let setup = TestSetup::default();
    let ledger = setup.ledger.canister_id();
    setup.upgrade_paid_service(Some(InitArgs { ledger }));
    assert_cost_1b_succeeds(&setup);
}
