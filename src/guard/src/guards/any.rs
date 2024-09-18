//! Accepts any payment that the vendor accepts.

use std::any::Any;

use ic_papi_api::{vendor::PaymentOption, PaymentType};

use super::PaymentGuard;

pub struct AnyPaymentGuard<const CAP: usize>{
    pub supported: [PaymentOption;CAP],
}

impl<const CAP: usize> AnyPaymentGuard<CAP> {
    pub fn deduct(_fee: u64, _payment: PaymentType) {
        unimplemented!()
    }
    pub fn 
}