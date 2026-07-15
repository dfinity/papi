//! Regression test: the paid service must remain usable after a canister upgrade.
//!
//! The init args (which include the payment ledger) are held in a non-stable `thread_local`.
//! Without a `post_upgrade` hook to restore them, `cost_1b` traps with "No init args provided"
//! after any upgrade. See the `pre_upgrade`/`post_upgrade` hooks in `src/lib.rs`.
use crate::util::test_environment::{PaidMethods, TestSetup, LEDGER_FEE};
use ic_papi_api::caller::CallerPaysIcrc2Tokens;
use ic_papi_api::cycles::cycles_ledger_canister_id;
use ic_papi_api::{PaymentError, PaymentType};

/// Verifies that `cost_1b` still works after the canister has been upgraded.
///
/// The upgrade happens before any call has lazily initialised the payment guard, so the guard is
/// first built (reading the init args) only after the upgrade. This is exactly the path that used
/// to trap when the init args were not persisted across upgrades.
#[test]
fn cost_1b_works_after_upgrade() {
    let setup = TestSetup::default();
    let method = PaidMethods::Cost1b;

    // Upgrade the canister. Without persistence, this would drop the init args.
    setup.upgrade_paid_service();

    // Pre-approve payment.
    setup.user_approves_payment_for_paid_service(method.cost() + LEDGER_FEE);

    // The call must still succeed: the payment ledger config survived the upgrade.
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
