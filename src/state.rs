use arc_swap::ArcSwap;
use std::sync::Arc;
use prometheus::Registry;
pub struct State {
    pub registry: Registry,
}

impl State {
    pub fn new(registry: Registry) -> Self {
        Self {
            registry,
        }
    }
}
