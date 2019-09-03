use actix_web::{App, HttpResponse, HttpServer, Responder, web};

use order_item::OrderItem;
use order_service::OrderService;

pub mod order_item;
pub mod order_service;


pub struct WebServer {}

impl WebServer {
    pub fn new() -> Self { WebServer {} }

    pub fn start(&self, port: u64) {
        let shared_data = web::Data::new(OrderService::new());

        HttpServer::new(move || {
            App::new()
                .register_data(shared_data.clone())
                .service(
                    web::scope("/tables/{id}")
                        .route("/order-items", web::get().to(WebServer::handle_get_items))
                        .route("/order-items", web::post().to(WebServer::handle_add_item))
                        .route("/order-items/{name}", web::delete().to(WebServer::handle_delete_item)),
                )
        })
            .bind(format!("127.0.0.1:{}", port).as_str())
            .expect(format!("Can not bind to port {}", port).as_str())
            .run()
            .unwrap();
    }

    fn handle_get_items(table_id: web::Path<u8>, stateful_service: web::Data<OrderService>) -> impl Responder {
        stateful_service
            .get_items(table_id.into_inner())
            .map(|items| web::Json(items))
    }

    fn handle_add_item((table_id, order_item): (web::Path<u8>, web::Json<OrderItem>), stateful_service: web::Data<OrderService>) -> impl Responder {
        let result = stateful_service.add(table_id.into_inner(), order_item.into_inner());
        match result {
            Ok(_) => HttpResponse::Ok(),
            _ => HttpResponse::BadRequest(),
        }
    }

    fn handle_delete_item(path_vars: web::Path<(u8, String)>, stateful_service: web::Data<OrderService>) -> impl Responder {
        let (table_id, item_name) = path_vars.into_inner();
        match stateful_service.cancel_item(table_id, item_name) {
            Ok(_) => HttpResponse::Ok(),
            _ => HttpResponse::BadRequest()
        }
    }
}

pub mod clients {
    use std::thread;
    use rand::Rng;
    use reqwest::Client;
    use std::time::Instant;
    use crate::order_item::OrderItem;

    pub struct LocalClient { base_url: String, client: Client }

    impl LocalClient {
        pub fn new(port: u64) -> Self {
            LocalClient {
                base_url: format!("http://localhost:{}", port),
                client: Client::new(),
            }
        }
        pub fn order_item(&self, table_id: u8, item_name: &str) {
            let url = format!("{}/tables/{}/order-items", self.base_url, table_id);
            let resp = self.client
                .post(url.as_str())
                .json(&OrderItem::new(item_name.to_string()))
                .send()
                .unwrap();
            assert!(resp.status().is_success());
        }

        pub fn get_items(&self, table_id: u8) -> Vec<OrderItem> {
            let url = format!("{}/tables/{}/order-items", self.base_url, table_id);
            let mut resp = self.client
                .get(url.as_str())
                .send()
                .unwrap();
            assert!(resp.status().is_success(), "fail to get items");
            let body: Vec<OrderItem> = resp.json().unwrap();
            body
        }

        pub fn cancel_order(&self, table_id: u8, item_name: &str) {
            let url = format!("{}/tables/{}/order-items/{}", self.base_url, table_id, item_name);
            let resp = self.client
                .delete(url.as_str())
                .send()
                .unwrap();
            assert!(resp.status().is_success(), "fail to delete item");
        }
    }

    pub fn clients_busy_with_orders(port: u64,
                                    n_thread_per_action: u64,
                                    n_action_per_thread: u64,
                                    table_range: (u8, u8)) {
        let mut thread_handles = vec![];
        let now = Instant::now();

        for _ in 0..n_thread_per_action {
            thread_handles.push(thread::spawn(move || {
                let client = LocalClient::new(port);
                for _ in 0..n_action_per_thread {
                    let table_id = rand::thread_rng().gen_range(table_range.0, table_range.1);
                    client.get_items(table_id);
                }
            }))
        }

        for _ in 0..n_thread_per_action {
            thread_handles.push(thread::spawn(move || {
                let client = LocalClient::new(port);
                for _ in 0..n_action_per_thread {
                    let table_id = rand::thread_rng().gen_range(table_range.0, table_range.1);
                    client.order_item(table_id, "bacon");
                }
            }))
        }

        for _ in 0..n_thread_per_action {
            thread_handles.push(thread::spawn(move || {
                let client = LocalClient::new(port);
                for _ in 0..n_action_per_thread {
                    let table_id = rand::thread_rng().gen_range(table_range.0, table_range.1);
                    client.cancel_order(table_id, "bacon");
                }
            }))
        }

        for h in thread_handles {
            h.join().unwrap();
        }
        let time_elapsed = now.elapsed().as_secs();

        let client = LocalClient::new(port);

        for i in (table_range.0)..(table_range.1 + 1) {
            println!("table {} has items: {:?}", i, client.get_items(i));
        }

        println!("Clients spent {}s on messing up the above orders.", time_elapsed);
    }
}


