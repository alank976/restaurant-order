use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::order_item::OrderItem;

pub struct OrderService(Arc<RwLock<HashMap<u8, Vec<OrderItem>>>>);

impl OrderService {
    pub fn new() -> Self {
        OrderService(Arc::new(RwLock::new(HashMap::new())))
    }

    fn new_for_test(m: Arc<RwLock<HashMap<u8, Vec<OrderItem>>>>) -> Self {
        OrderService(m)
    }

    pub fn add(&self, table_id: u8, item: OrderItem) -> Result<(), ()> {
        match table_id {
            1..=100 => {
                let mut items_by_table_id = self.0
                    .write()
                    .unwrap();
                if let Some(items) = items_by_table_id.get_mut(&table_id) {
                    items.push(item);
                    Ok(())
                } else {
                    items_by_table_id.insert(table_id, vec![item]);
                    Ok(())
                }
            }
            _ => Err(())
        }
    }

    pub fn get_items(&self, table_id: u8) -> Result<Vec<OrderItem>, ()> {
        match table_id {
            1..=100 => Ok(
                self.0
                    .read()
                    .unwrap()
                    .get(&table_id)
                    .map(|it| it.clone())
                    .unwrap_or(vec![])
            ),
            _ => Err(())
        }
    }

    pub fn cancel_item(&self, table_id: u8, item_name: String) -> Result<(), ()> {
        match table_id {
            1..=100 => {
                if let Some(items) = self.0
                    .write()
                    .unwrap()
                    .get_mut(&table_id) {
                    items.retain(|item| item_name != *item.name);
                }
                Ok(())
            }
            _ => Err(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_service_and_inner_map() -> (OrderService, Arc<RwLock<HashMap<u8, Vec<OrderItem>>>>) {
        let m: HashMap<u8, Vec<OrderItem>> = HashMap::new();
        let arc_rw = Arc::new(RwLock::new(m));
        let svc = OrderService::new_for_test(arc_rw.clone());
        (svc, arc_rw.clone())
    }

    #[test]
    fn it_returns_items_from_hashmap() {
        let (svc, rw_map) = new_service_and_inner_map();
        rw_map
            .write()
            .unwrap()
            .insert(10, vec![OrderItem::new("sushi".to_string())]);

        let items = svc.get_items(10);
        assert!(items.is_ok());
        let items = items.unwrap();
        assert_eq!(1, items.len());
        let item = items.get(0).unwrap();
        assert_eq!("sushi".to_string(), item.name);
    }

    #[test]
    fn it_returns_nothing_when_no_table_id_found() {
        let (svc, _) = new_service_and_inner_map();

        let items = svc.get_items(2);

        assert!(items.is_ok());
        assert!(items.unwrap().is_empty());
    }

    #[test]
    fn it_saves_item_into_map() {
        let (svc, rw_map) = new_service_and_inner_map();

        let result = svc.add(1, OrderItem::new("french fries".to_string()));
        assert!(result.is_ok());
        let inner_map = rw_map
            .read()
            .unwrap();

        let items = inner_map.get(&1);
        assert!(items.is_some());
        let items = items.unwrap();
        assert_eq!(1, items.len());
        assert_eq!("french fries",
                   items
                       .first()
                       .unwrap()
                       .name()
        );
    }

    #[test]
    fn it_saves_items_with_same_name() {
        let (svc, rw_map) = new_service_and_inner_map();

        for _ in 0..2 {
            let result = svc.add(1, OrderItem::new("french fries".to_string()));
            assert!(result.is_ok());
        }
        let inner_map = rw_map
            .read()
            .unwrap();
        let mut items = inner_map.get(&1);
        assert!(items.is_some());
        let items = items.as_mut().unwrap();
        assert_eq!(2, items.len());
        assert!(items.iter()
            .map(|item: &OrderItem| item.name())
            .all(|name| "french fries" == name));
    }

    #[test]
    fn it_deletes_item() {
        let (svc, rw_map) = new_service_and_inner_map();

        rw_map
            .write()
            .unwrap()
            .insert(1, vec![OrderItem::new("ramen".to_string())]);
        svc.cancel_item(1, "ramen".to_string()).unwrap();

        assert!(rw_map
            .read()
            .unwrap()
            .get(&1)
            .unwrap()
            .is_empty());
    }

    #[test]
    fn it_rejects_when_table_id_greater_100() {
        let (svc, _) = new_service_and_inner_map();

        let items = svc.get_items(200);
        assert!(items.is_err());
    }
}