//! Payment API error types.
use candid::{CandidType, Deserialize, Principal};
pub use ic_cycles_ledger_client::Account;
use ic_cycles_ledger_client::{TransferFromError, WithdrawFromError};

use crate::caller::TokenAmount;

#[derive(Debug, CandidType, Deserialize, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentError {
    UnsupportedPaymentType,
    LedgerUnreachable {
        ledger: Principal,
    },
    LedgerWithdrawFromError {
        ledger: Principal,
        error: WithdrawFromError,
    },
    LedgerTransferFromError {
        ledger: Principal,
        error: TransferFromError,
    },
    InsufficientFunds {
        needed: TokenAmount,
        available: TokenAmount,
    },
    InvalidPatron,
}

impl std::fmt::Display for PaymentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PaymentError::UnsupportedPaymentType => write!(f, "Unsupported payment type"),
            PaymentError::LedgerUnreachable { ledger } => {
                write!(f, "Ledger unreachable: {ledger}")
            }
            PaymentError::LedgerWithdrawFromError { ledger, error } => {
                write!(f, "Ledger {ledger} withdraw_from error: {error:?}")
            }
            PaymentError::LedgerTransferFromError { ledger, error } => {
                write!(f, "Ledger {ledger} transfer_from error: {error:?}")
            }
            PaymentError::InsufficientFunds { needed, available } => {
                write!(
                    f,
                    "Insufficient funds: needed {needed}, available {available}"
                )
            }
            PaymentError::InvalidPatron => write!(f, "Invalid patron"),
        }
    }
}
