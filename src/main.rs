use restaurant_order::WebServer;
use std::thread;
use restaurant_order::clients::clients_busy_with_orders;


fn main() {
    raw_run();
}

fn raw_run() {
    WebServer::new().start(8000)
}

fn run_with_client() {
    thread::spawn(move || WebServer::new().start(8000));

    clients_busy_with_orders(8000, 100, 500, (1, 10));
}