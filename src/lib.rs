use std::sync::mpsc::channel;
use std::thread;

use actix_web::{App, HttpResponse, HttpServer, Responder, web};

use order_item::OrderItem;
use order_service::OrderService;

pub mod order_item;
pub mod order_service;


pub struct WebServer {}

impl WebServer {
    pub fn new() -> Self { WebServer {} }

    pub fn start(&self) {
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
            .bind("127.0.0.1:8000")
            .expect("Can not bind to port 8000")
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


