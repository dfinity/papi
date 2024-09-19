//! Accepts any payment that the vendor accepts.

use candid::{CandidType, Deserialize};
use ic_papi_api::{caller::TokenAmount, PaymentError, PaymentType};

use super::{attached_cycles::AttachedCyclesPayment, PaymentContext, PaymentGuard, PaymentGuard2};

/// A guard that accepts a user-specified payment type, providing the vendor supports it.
pub struct AnyPaymentGuard<const CAP: usize> {
    pub supported: [VendorPaymentConfig; CAP],
}

/// Vendor payment configuration, including details that may not necessarily be shared with the customer.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum VendorPaymentConfig {
    /// Cycles are received by the vendor canister.
    AttachedCycles,
    /// Cycles are received by the vendor canister.
    CallerPaysIcrc2Cycles,
    /// Cycles are received by the vendor canister.
    PatronPaysIcrc2Cycles,
}

impl<const CAP: usize> PaymentGuard2 for AnyPaymentGuard<CAP> {
    async fn deduct(
        &self,
        _context: PaymentContext,
        payment: PaymentType,
        fee: TokenAmount,
    ) -> Result<(), PaymentError> {
        let payment_config = self
            .config(payment)
            .ok_or(PaymentError::UnsupportedPaymentType)?;
        match payment_config {
            VendorPaymentConfig::AttachedCycles => AttachedCyclesPayment {}.deduct(fee).await,
            VendorPaymentConfig::CallerPaysIcrc2Cycles => unimplemented!(),
            VendorPaymentConfig::PatronPaysIcrc2Cycles => unimplemented!(),
        }
    }
}
impl<const CAP: usize> AnyPaymentGuard<CAP> {
    /// Find the vendor configuration for the offered payment type.
    fn config(&self, payment: PaymentType) -> Option<&VendorPaymentConfig> {
        match payment {
            PaymentType::AttachedCycles => self
                .supported
                .iter()
                .find(|&x| *x == VendorPaymentConfig::AttachedCycles),
            PaymentType::CallerPaysIcrc2Cycles => self
                .supported
                .iter()
                .find(|&x| *x == VendorPaymentConfig::CallerPaysIcrc2Cycles),
            PaymentType::PatronPaysIcrc2Cycles(_) => self
                .supported
                .iter()
                .find(|&x| *x == VendorPaymentConfig::PatronPaysIcrc2Cycles),
            _ => None,
        }
    }
}
