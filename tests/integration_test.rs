#[cfg(test)]
mod tests {
    use std::thread;
    use std::time::{Duration, Instant};

    use restaurant_order::*;
    use restaurant_order::clients::*;

    #[test]
    fn integration_test() {
        thread::spawn(move || WebServer::new().start(8001));

        thread::sleep(Duration::from_secs(1));

        let client = LocalClient::new(8001);
        assert!(client.get_items(1).is_empty(), "items ordered already in the beginning");

        // order a breakfast
        client.order_item(1, "bacon_and_eggs");

        // check if ordered breakfast is there
        assert_eq!(1, client.get_items(1).len());

        // customer just changes his/her mind
        client.cancel_order(1, "bacon_and_eggs");

        // this customer has ordered nothing now
        assert!(client.get_items(1).is_empty(), "item has not been deleted");
    }

    #[test]
    fn load_test() {
        thread::spawn(move || WebServer::new().start(8002));

        let now = Instant::now();
        clients_busy_with_orders(8002, 10, 5, (1, 10));
        let elapsed_time = now.elapsed().as_secs();
        assert!(elapsed_time < 1);
    }
}
