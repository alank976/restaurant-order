#[cfg(test)]
mod tests {
    // use super::*;
    use restaurant_order::*;
    use actix_web::dev::Service;
    use actix_web::{test, web, App};

    #[test]
    fn test_get_table_order_items() {
        let mut app = test::init_service(App::new().service(get_order_items));
        let req = test::TestRequest::get().uri("/tables/2/order-items").to_request();
        let resp = test::block_on(app.call(req)).unwrap();

        assert!(resp.status().is_success());
    }

}