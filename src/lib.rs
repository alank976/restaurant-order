use actix_web::get;
use actix_web::{web, App, HttpRequest, HttpServer, Responder};

pub fn start_web_server() {
    HttpServer::new(|| App::new().service(get_order_items))
        .bind("127.0.0.1:8000")
        .expect("Can not bind to port 8000")
        .run()
        .unwrap();
}

#[get("/tables/{id}/order-items")]
pub fn get_order_items(req: HttpRequest) -> impl Responder {
    let table_id: u8 = req
        .match_info()
        .get("id")
        .and_then(|s: &str| s.parse::<u8>().ok())
        .unwrap();
    let table = domain::Table::new(table_id);
    web::Json(table)
}

mod domain {
    use serde::Serialize;
    use std::time::Duration;

    #[derive(Serialize)]
    pub struct OrderItem {
        name: String,
        time_to_cook: Duration,
        table_id: u8,
    }

    #[derive(Serialize)]
    pub struct Table {
        id: u8,
        order_items: Vec<OrderItem>,
    }

    impl Table {
        pub fn new(id: u8) -> Self {
            Table {
                id,
                order_items: vec![],
            }
        }
    }
}
