use std::cell::RefCell;
use lazy_static::lazy_static;
use candid::Principal;
use example_paid_service_api::InitArgs;
use ic_papi_guard::guards::any::{AnyPaymentGuard, VendorPaymentConfig};

thread_local! {
    pub static INIT_ARGS: RefCell<Option<InitArgs>> = const {RefCell::new(None)};
}
lazy_static! {
    pub static ref PAYMENT_GUARD: AnyPaymentGuard<5> = AnyPaymentGuard {
        supported: [
            VendorPaymentConfig::AttachedCycles,
            VendorPaymentConfig::CallerPaysIcrc2Cycles,
            VendorPaymentConfig::PatronPaysIcrc2Cycles,
            VendorPaymentConfig::CallerPaysIcrc2Tokens {
                ledger: payment_ledger(),
            },
            VendorPaymentConfig::PatronPaysIcrc2Tokens {
                ledger: payment_ledger(),
            },
        ],
    };
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
}