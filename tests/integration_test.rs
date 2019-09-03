#[cfg(test)]
mod tests {
    use std::thread;
    use std::time::{Duration, Instant};


    use restaurant_order::*;
    use restaurant_order::clients::*;

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
        let server_thread = thread::spawn(move || WebServer::new().start());

        let mut thread_handles = vec![];
        let client = reqwest::Client::new();
        let base_url = "http://localhost:8000";

        let now = Instant::now();
        clients_busy_with_orders(10, 5, (1, 10));
        let elapsed_time = now.elapsed().as_secs();
        assert!(elapsed_time < 1);
    }
}
