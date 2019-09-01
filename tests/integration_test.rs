#[cfg(test)]
mod tests {
    use restaurant_order::*;

    use reqwest;
    use restaurant_order::order_item::OrderItem;
    use std::borrow::Borrow;
    use reqwest::Client;


    #[test]
    fn integration_test() {
        let server = WebServer::new().start();
        let url = "http://localhost:8000/tables/1/order-items";
        let client = reqwest::Client::new();

        assert!(get_items(url, &client).is_empty(), "items ordered already in the beginning");

        // order a breakfast
        order_item(url, &client, "bacon_and_eggs");

        // check if ordered breakfast is there
        assert_eq!(1, get_items(url, &client).len());

        // customer just changes his/her mind
        cancel_order(url, &client, "bacon_and_eggs");

        // this customer has ordered nothing now
        assert!(get_items(url, &client).is_empty(), "item has not been deleted");

        server.stop(true);
    }

    fn order_item(url: &str, client: &Client, item_name: &str) {
        let resp = client
            .post(url)
            .json(&OrderItem::new(item_name.to_string()))
            .send()
            .unwrap();
        assert!(resp.status().is_success());
    }

    fn get_items(url: &str, client: &Client) -> Vec<OrderItem> {
        let mut resp = client
            .get(url)
            .send()
            .unwrap();
        assert!(resp.status().is_success(), "fail to get items");
        let body: Vec<OrderItem> = resp.json().unwrap();
        body
    }

    fn cancel_order(url: &str, client: &Client, item_name: &str) {
        let url = format!("{}/{}", url, item_name);
        let resp = client
            .delete(url.as_str())
            .send()
            .unwrap();
        assert!(resp.status().is_success(), "fail to delete item");
    }
}
