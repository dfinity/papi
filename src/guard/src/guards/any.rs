//! Accepts any payment that the vendor accepts.

use ic_papi_api::{caller::{PatronPaysIcrc2Cycles, TokenAmount}, cycles::cycles_ledger_canister_id, principal2account, Account, PaymentError, PaymentType};

use super::{attached_cycles::AttachedCyclesPayment, icrc2_cycles::Icrc2CyclesPaymentGuard, PaymentContext, PaymentGuard, PaymentGuard2};

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

/// A user's requested payment type paired with a vendor's configuration.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PaymentWithConfig {
     AttachedCycles,
        CallerPaysIcrc2Cycles,
        PatronPaysIcrc2Cycles(PatronPaysIcrc2Cycles),
}


impl<const CAP: usize> PaymentGuard2 for AnyPaymentGuard<CAP> {
    async fn deduct(
        &self,
        context: PaymentContext,
        payment: PaymentType,
        fee: TokenAmount,
    ) -> Result<(), PaymentError> {
        let PaymentContext{caller, own_canister_id} = context;
        let payment_config = self
            .config(payment)
            .ok_or(PaymentError::UnsupportedPaymentType)?;
        match payment_config {
            PaymentWithConfig::AttachedCycles => AttachedCyclesPayment {}.deduct(fee).await,
            PaymentWithConfig::CallerPaysIcrc2Cycles => Icrc2CyclesPaymentGuard {
                ledger_canister_id: cycles_ledger_canister_id(),
                payer_account: Account{owner: caller, subaccount: None},
                spender_subaccount: None,
                created_at_time: None,
                own_canister_id,
            }.deduct(fee).await,
            PaymentWithConfig::PatronPaysIcrc2Cycles(patron) => Icrc2CyclesPaymentGuard {
                ledger_canister_id: cycles_ledger_canister_id(),
                payer_account: patron,
                spender_subaccount: Some(principal2account(&caller)),
                ..Icrc2CyclesPaymentGuard::default()
            }.deduct(fee).await,
        }
    }
}
impl<const CAP: usize> AnyPaymentGuard<CAP> {
    /// Find the vendor configuration for the offered payment type.
    fn config(&self, payment: PaymentType) -> Option<PaymentWithConfig> {
        match payment {
            PaymentType::AttachedCycles => self
                .supported
                .iter()
                .find(|&x| *x == VendorPaymentConfig::AttachedCycles).map(|_| PaymentWithConfig::AttachedCycles),
            PaymentType::CallerPaysIcrc2Cycles => self
                .supported
                .iter()
                .find(|&x| *x == VendorPaymentConfig::CallerPaysIcrc2Cycles).map(|_| PaymentWithConfig::CallerPaysIcrc2Cycles),
            PaymentType::PatronPaysIcrc2Cycles(patron) => self
                .supported
                .iter()
                .find(|&x| *x == VendorPaymentConfig::PatronPaysIcrc2Cycles).map(|_| PaymentWithConfig::PatronPaysIcrc2Cycles(patron)),
            _ => None,
        }
    }
}
