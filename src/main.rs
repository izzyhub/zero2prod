use zero2prod::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let address = "127.0.0.1:8000";
    let listener = TcpListener::bind(address).expect(&format!("Could not bind {address}"));
    run(listener)?.await
}
