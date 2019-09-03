#[cfg(test)]
mod tests {
    use std::thread;
    use std::time::Duration;

    use rand::Rng;
    use reqwest;
    use reqwest::Client;

    use restaurant_order::*;
    use restaurant_order::order_item::OrderItem;

    #[test]
    fn integration_test() {
        let server_thread = thread::spawn(move || WebServer::new().start());

        thread::sleep(Duration::from_secs(1));

        let base_url = "http://localhost:8000";
        let client = reqwest::Client::new();

        assert!(get_items(base_url, &client, 1).is_empty(), "items ordered already in the beginning");

        // order a breakfast
        order_item(base_url, &client, 1, "bacon_and_eggs");

        // check if ordered breakfast is there
        assert_eq!(1, get_items(base_url, &client, 1).len());

        // customer just changes his/her mind
        cancel_order(base_url, &client, 1, "bacon_and_eggs");

        // this customer has ordered nothing now
        assert!(get_items(base_url, &client, 1).is_empty(), "item has not been deleted");
    }

    #[test]
    fn load_test() {
        let mut thread_handles = vec![];
        let client = reqwest::Client::new();
        let base_url = "http://localhost:8000";

        let nthread = 2;

        for _ in 0..nthread {
            thread_handles.push(thread::spawn(move || {
                let client = reqwest::Client::new();
                let base_url = "http://localhost:8000";
                for _ in 0..5 {
                    let table_id = rand::thread_rng().gen_range(1, 10);
                    order_item(base_url, &client, table_id, "bacon");
                }
            }))
        }
        for _ in 0..nthread {
            thread_handles.push(thread::spawn(move || {
                let client = reqwest::Client::new();
                let base_url = "http://localhost:8000";
                for _ in 0..5 {
                    let table_id = rand::thread_rng().gen_range(1, 10);
                    get_items(base_url, &client, table_id);
                }
            }))
        }
        for _ in 0..nthread {
            thread_handles.push(thread::spawn(move || {
                let client = reqwest::Client::new();
                let base_url = "http://localhost:8000";
                for _ in 0..5 {
                    let table_id = rand::thread_rng().gen_range(1, 10);
                    cancel_order(base_url, &client, table_id, "bacon")
                }
            }))
        }

        for h in thread_handles {
            h.join().unwrap();
        }

        for i in 1..11 {
            println!("table {} has items: {:?}", i, get_items(base_url, &client, i));
        }
    }

    fn order_item(base_url: &str, client: &Client, table_id: u8, item_name: &str) {
        let url = format!("{}/tables/{}/order-items", base_url, table_id);
        let resp = client
            .post(url.as_str())
            .json(&OrderItem::new(item_name.to_string()))
            .send()
            .unwrap();
        assert!(resp.status().is_success());
    }

    fn get_items(base_url: &str, client: &Client, table_id: u8) -> Vec<OrderItem> {
        let url = format!("{}/tables/{}/order-items", base_url, table_id);
        let mut resp = client
            .get(url.as_str())
            .send()
            .unwrap();
        assert!(resp.status().is_success(), "fail to get items");
        let body: Vec<OrderItem> = resp.json().unwrap();
        body
    }

    fn cancel_order(base_url: &str, client: &Client, table_id: u8, item_name: &str) {
        let url = format!("{}/tables/{}/order-items/{}", base_url, table_id, item_name);
        let resp = client
            .delete(url.as_str())
            .send()
            .unwrap();
        assert!(resp.status().is_success(), "fail to delete item");
    }
}
