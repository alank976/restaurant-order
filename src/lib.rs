use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use actix_rt;

use order_item::OrderItem;
use std::sync::mpsc::channel;
use std::thread;
use actix_web::dev::Server;

pub mod order_item;

pub struct OrderService(Arc<RwLock<HashMap<u8, Vec<OrderItem>>>>);

impl OrderService {
    pub fn new() -> Self {
        OrderService(Arc::new(RwLock::new(HashMap::new())))
    }

    fn add(&self, table_id: u8, item: OrderItem) -> Result<(), ()> {
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

    fn get_items(&self, table_id: u8) -> Option<Vec<OrderItem>> {
        self.0
            .read()
            .unwrap()
            .get(&table_id)
            .map(|it| it.clone())
    }

    fn cancel_item(&self, table_id: u8, item_name: String) {
        if let Some(items) = self.0
            .write()
            .unwrap()
            .get_mut(&table_id) {
            items.retain(|item| item_name != *item.name);
        }
    }
}

pub struct WebServer {}

impl WebServer {
    pub fn new() -> Self { WebServer {} }

    pub fn start(&self) -> Server {
        let shared_data = web::Data::new(OrderService::new());

        let (tx, rx) = channel();

        thread::spawn(move || {
            let sys = actix_rt::System::new("example");
            let server = HttpServer::new(move || {
                App::new()
                    .register_data(shared_data.clone())
                    .configure(WebServer::table_scope_config)
            })
                .bind("127.0.0.1:8000")
                .expect("Can not bind to port 8000")
                .start();
            let _ = tx.send(server);
            let _ = sys.run();
        });
        rx.recv().unwrap()
    }

    pub fn table_scope_config(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::scope("/tables/{id}")
                .route("/order-items", web::get().to(WebServer::get_items_handler))
                .route("/order-items", web::post().to(WebServer::add_item_handler))
                .route("/order-items/{name}", web::delete().to(WebServer::delete_item_handler)),
        );
    }

    fn get_items_handler(table_id: web::Path<u8>, stateful_service: web::Data<OrderService>) -> impl Responder {
        stateful_service
            .get_items(table_id.into_inner())
            .map(|item| web::Json(item))
            .unwrap_or_else(|| web::Json(vec![]))
    }

    fn add_item_handler((table_id, order_item): (web::Path<u8>, web::Json<OrderItem>), stateful_service: web::Data<OrderService>) -> impl Responder {
        let result = stateful_service.add(table_id.into_inner(), order_item.into_inner());
        match result {
            Ok(()) => HttpResponse::Ok(),
            _ => HttpResponse::InternalServerError(),
        }
    }

    fn delete_item_handler(path_vars: web::Path<(u8, String)>, stateful_service: web::Data<OrderService>) -> impl Responder {
        let (table_id, item_name) = path_vars.into_inner();
        stateful_service.cancel_item(table_id, item_name);
        HttpResponse::Ok()
    }
}


