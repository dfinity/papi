use ic_cdk_macros::update;

#[update()]
async fn free() -> String {
    "Yes, I am free!".to_string()
}
