use candid::Principal;
pub use cycles_ledger_client::Account;
use serde_bytes::ByteBuf;

pub mod caller;
pub mod error;
pub mod vendor;
pub use caller::PaymentType;
pub use error::PaymentError;
pub use vendor::Icrc2Payer;

#[must_use]
pub fn principal2account(principal: &Principal) -> ByteBuf {
    // TODO: This is NOT the right way.
    let mut ans = principal.as_slice().to_vec();
    while ans.len() < 32 {
        ans.push(0);
    }
    ByteBuf::from(ans)
}
