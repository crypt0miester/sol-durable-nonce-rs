use crate::use_storage::*;


pub fn get_durable_nonce(user_key: &str) -> Option<String> {
    let key = format!("durableNonce_{}", user_key);
    local_storage::get_item(&key)
}

pub fn set_durable_nonce(user_key: &str, durable_nonce_key: &str) {
    let key = format!("durableNonce_{}", user_key);
    local_storage::set_item(&key, durable_nonce_key);
}