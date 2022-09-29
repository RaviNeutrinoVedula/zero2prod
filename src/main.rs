use zero2prod::{startup::run, configuration::{self, get_configuration}} ;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Panic if we can't read the configuratoin
    let configuration = get_configuration()
        .expect("Failed to read configuration");

    // We have removed the hard-coded port# 8000; it's now coming from settings
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener)?.await
}
