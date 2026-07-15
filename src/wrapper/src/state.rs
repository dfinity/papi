//! Operator-controlled configuration state for the wrapper.
//!
//! The wrapper prices each proxied `(target, method)` server-side via a
//! [`MethodConfig`]. Only the wrapper's controllers may change this
//! configuration; callers can never set their own fee or forwarded-cycle amount.
//!
//! State is held on the heap and persisted across upgrades in stable memory via
//! the `pre_upgrade`/`post_upgrade` hooks in `lib.rs`.

use crate::domain::types::{MethodConfig, MethodKey};
use std::cell::RefCell;
use std::collections::HashMap;

thread_local! {
    static CONFIGS: RefCell<HashMap<MethodKey, MethodConfig>> = RefCell::new(HashMap::new());
}

/// Look up the operator configuration for a `(target, method)` pair.
#[must_use]
pub fn get_config(key: &MethodKey) -> Option<MethodConfig> {
    CONFIGS.with(|c| c.borrow().get(key).cloned())
}

/// Insert or replace the configuration for a `(target, method)` pair.
pub fn set_config(key: MethodKey, config: MethodConfig) {
    CONFIGS.with(|c| {
        c.borrow_mut().insert(key, config);
    });
}

/// Remove the configuration for a `(target, method)` pair, returning any prior value.
#[must_use]
pub fn remove_config(key: &MethodKey) -> Option<MethodConfig> {
    CONFIGS.with(|c| c.borrow_mut().remove(key))
}

/// Snapshot of all configured `(target, method)` prices.
#[must_use]
pub fn list_configs() -> Vec<(MethodKey, MethodConfig)> {
    CONFIGS.with(|c| {
        c.borrow()
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    })
}

/// Replace the whole registry (used when restoring after an upgrade).
pub fn replace_all(items: Vec<(MethodKey, MethodConfig)>) {
    CONFIGS.with(|c| {
        let mut map = c.borrow_mut();
        map.clear();
        for (k, v) in items {
            map.insert(k, v);
        }
    });
}
