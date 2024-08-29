
use ic_cdk_macros::update;


#[update()]
async fn caller_eth_address() -> String {
    "Hello world!".to_string()
}