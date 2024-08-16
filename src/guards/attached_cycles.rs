use super::{PaymentError, PaymentGuard};
use ic_cdk::api::call::{msg_cycles_accept, msg_cycles_available};

pub struct AttachedCyclesPayment {}

impl PaymentGuard for AttachedCyclesPayment {
    fn deduct(fee: u64) -> Result<(), PaymentError> {
        let available = msg_cycles_available();
        if available < fee {
            ic_cdk::trap("Not enough cycles attached");
        }
        msg_cycles_accept(fee);
        Ok(())
    }
}
