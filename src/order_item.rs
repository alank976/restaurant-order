use serde::Serialize;
use serde::Deserialize;
use std::time::Duration;
use rand::Rng;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct OrderItem {
    pub(crate) name: String,
    time_to_cook: Duration,
}

impl OrderItem {
    pub fn new(name: String) -> Self {
        let cook_mins = rand::thread_rng().gen_range(5, 15);
        OrderItem {
            name,
            time_to_cook: Duration::from_secs(60 * cook_mins),
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_randomize_cook_time() {
        let min = Duration::from_secs(60 * 5);
        let max = Duration::from_secs(60 * 15);


        let item = OrderItem::new("ramen".to_string());
        let time_to_cook = item.time_to_cook;

        assert!(time_to_cook <= max && time_to_cook >= min);
    }
}