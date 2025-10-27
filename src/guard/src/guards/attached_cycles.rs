use super::{PaymentError, PaymentGuardTrait};
use ic_cdk::api::{msg_cycles_accept, msg_cycles_available};
use ic_papi_api::caller::TokenAmount;

/// The information required to charge attached cycles.
#[derive(Default, Debug, Eq, PartialEq)]
pub struct AttachedCyclesPayment {}

impl PaymentGuardTrait for AttachedCyclesPayment {
    async fn deduct(&self, fee: TokenAmount) -> Result<(), PaymentError> {
        let available = msg_cycles_available();
        if available < fee {
            return Err(PaymentError::InsufficientFunds {
                needed: fee,
                available,
            });
        }
        msg_cycles_accept(fee);
        Ok(())
    }
}
