use super::{PaymentError, PaymentGuardTrait};
use ic_cdk::api::{msg_cycles_accept, msg_cycles_available};
use ic_papi_api::caller::TokenAmount;

/// The information required to charge attached cycles.
#[derive(Default, Debug, Eq, PartialEq)]
pub struct AttachedCyclesPayment {}

impl PaymentGuardTrait for AttachedCyclesPayment {
    async fn deduct(&self, fee: TokenAmount) -> Result<(), PaymentError> {
        let available = msg_cycles_available();
        let fee_u128 = u128::from(fee);
        if available < fee_u128 {
            return Err(PaymentError::InsufficientFunds {
                needed: fee,
                available: available.try_into().unwrap_or(u64::MAX),
            });
        }
        msg_cycles_accept(fee_u128);
        Ok(())
    }
}
