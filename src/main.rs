use restaurant_order::WebServer;
use std::thread;
use restaurant_order::clients::clients_busy_with_orders;


fn main() {
    run_with_client();
}

fn raw_run() {
    WebServer::new().start()
}

fn run_with_client() {
    thread::spawn(move || WebServer::new().start());

    clients_busy_with_orders(100, 500, (1, 10));
}