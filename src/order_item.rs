use serde::Serialize;
use serde::Deserialize;
use std::time::Duration;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct OrderItem {
    pub(crate) name: String,
    time_to_cook: Duration,
}

impl OrderItem {
    pub fn new(name: String) -> Self {
        OrderItem {
            name,
            time_to_cook: Duration::from_secs(60),
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}
