//! Accepts any payment that the vendor accepts.

use ic_papi_api::vendor::PaymentOption;

pub struct AnyPaymentGuard<const CAP: usize>{
    pub supported: [PaymentOption;CAP],
}