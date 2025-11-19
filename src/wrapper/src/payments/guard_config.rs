use candid::Principal;
use ic_papi_guard::guards::any::{PaymentGuard, VendorPaymentConfig};
use lazy_static::lazy_static;

/// Return the ICRC-2 ledger principal used when the *payment type* is token-based.
///
/// Replace this with your real ledger principal.
fn payment_ledger() -> Principal {
    Principal::from_text("aaaaa-aa").unwrap() // TODO: set real ledger
}

/// Shared guard accepting multiple payment modes.
///
/// The *caller* selects the mode by passing an appropriate `PaymentType` variant.
/// The fee *amount* is provided per call.
lazy_static! {
    pub static ref PAYMENT_GUARD: PaymentGuard<5> = PaymentGuard {
        supported: [
            VendorPaymentConfig::AttachedCycles,
            VendorPaymentConfig::CallerPaysIcrc2Cycles,
            VendorPaymentConfig::PatronPaysIcrc2Cycles,
            VendorPaymentConfig::CallerPaysIcrc2Tokens { ledger: payment_ledger() },
            VendorPaymentConfig::PatronPaysIcrc2Tokens { ledger: payment_ledger() },
        ],
    };
}
