use candid::Principal;
use ic_papi_guard::guards::any::{PaymentGuard, VendorPaymentConfig};
use std::sync::LazyLock;

/// Return the ICRC-2 ledger principal used when the *payment type* is token-based.
///
/// Replace this with your real ledger principal.
fn payment_ledger() -> Principal {
    Principal::from_text("aaaaa-aa").unwrap() // TODO: set real ledger
}

pub static PAYMENT_GUARD: LazyLock<PaymentGuard<5>> = LazyLock::new(|| PaymentGuard {
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
});
