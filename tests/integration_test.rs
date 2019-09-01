#[cfg(test)]
mod tests {
    use super::*;
    // use actix_web::dev::{Service, ServiceResponse};
    // use actix_web::{test, web, App, Error};
    use restaurant_order::*;

    extern crate reqwest;


    #[test]
    fn test_get_table_order_items() {
//        let service = OrderService::new();
//
//        let mut app = test::init_service(
//            App::new()
//                .register_data(web::Data::new(service))
//                .configure(WebServer::table_scope_config));
////        let req = test::TestRequest::get()
////            .uri("/tables/2/order-items")
////            .to_request();
//
////        let resp = test::block_on(app.call(req))
////            .unwrap();
////        assert!(resp.status().is_success());
//
//
//        let req = test::TestRequest::get()
//            .uri("/tables/2/order-items")
//            .to_request();
//        let resp_body: String = test::read_response_json(&mut app, req);
//        assert_eq!("[]", resp_body);

        let web_server = WebServer::new();
        web_server.start();


        let resp = reqwest::get("http://localhost:8000/tables/1/order-items")?;
        assert!(resp.status().is_success());
    }
}
