use serde::Serialize;
use serde::Deserialize;
use std::time::Duration;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct OrderItem {
    pub(crate) name: String,
    time_to_cook: Duration,
}

impl OrderItem {
    pub fn new(name: String, time_to_cook: Duration) -> Self {
        OrderItem {
            name,
            time_to_cook,
        }
    }
}
