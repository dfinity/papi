use std::cell::RefCell;

use candid::Principal;

thread_local! {
    pub static PAYMENT_LEDGER: RefCell<Option<Principal>> = RefCell::new(None);
}

/// Provides the canister_id of he ledger used for payments.
pub fn payment_ledger() -> Principal {
    PAYMENT_LEDGER.with(|ledger| {
        ledger
            .borrow()
            .clone()
            .expect("Payment ledger was not set when the canister was initialized")
    })
}

/// Sets the payment ledger canister ID.
pub fn set_payment_ledger(ledger: Principal) {
    PAYMENT_LEDGER.set(Some(ledger));
}
