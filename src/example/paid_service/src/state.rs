use std::cell::RefCell;

use candid::Principal;
use example_paid_service_api::InitArgs;
use ic_papi_guard::guards::any::{AnyPaymentGuard, VendorPaymentConfig};

thread_local! {
    pub static INIT_ARGS: RefCell<Option<InitArgs>> = const {RefCell::new(None)};
    pub static PAYMENT_OPTIONS: RefCell<Option<AnyPaymentGuard<5>>> = const {RefCell::new(None)};
}

pub fn init_element<F, T>(f: F) -> T
where
    F: FnOnce(&InitArgs) -> T,
{
    INIT_ARGS.with(|init_args| f(init_args.borrow().as_ref().expect("No init args provided")))
}

/// Provides the canister id of the ledger used for payments.
pub fn payment_ledger() -> Principal {
    init_element(|init_args| init_args.ledger)
}

/// Sets the payment ledger canister ID.
pub fn set_init_args(init_args: InitArgs) {
    INIT_ARGS.set(Some(init_args.clone()));
    let guard = AnyPaymentGuard {
        supported: [
            VendorPaymentConfig::AttachedCycles,
            VendorPaymentConfig::CallerPaysIcrc2Cycles,
            VendorPaymentConfig::PatronPaysIcrc2Cycles,
            VendorPaymentConfig::CallerPaysIcrc2Tokens {
                ledger: init_args.ledger.clone(),
            },
            VendorPaymentConfig::PatronPaysIcrc2Tokens {
                ledger: init_args.ledger,
            },
        ],
    };
    PAYMENT_OPTIONS.set(Some(guard));
}
