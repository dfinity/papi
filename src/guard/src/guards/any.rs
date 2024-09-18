//! Accepts any payment that the vendor accepts.


use ic_papi_api::{vendor::VendorPaymentConfig, PaymentType};

pub struct AnyPaymentGuard<const CAP: usize>{
    pub supported: [VendorPaymentConfig;CAP],
}

impl<const CAP: usize> AnyPaymentGuard<CAP> {
    pub fn deduct(_fee: u64, _payment: PaymentType) {
        unimplemented!()
    }
}