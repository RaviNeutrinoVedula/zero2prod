use zero2prod::{startup::run, configuration::get_configuration};
use std::net::TcpListener;
use sqlx::postgres::PgPoolOptions;
use zero2prod::telemetry::{get_subscriber, init_subscriber};
use secrecy::ExposeSecret;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    
    // Panic if we can't read the configuratoin
    let configuration = get_configuration()
        .expect("Failed to read configuration");

    // let connection = PgConnection::connect(&configuration.database.connection_string())
    let connection_pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(5))
        .connect_lazy(&configuration.database.connection_string().expose_secret())
    .expect("Failed to create Postgres connection pool.");

    // We have removed the hard-coded port# 8000; it's now coming from settings
    // let address = format!("127.0.0.1:{}", configuration.application_port);
    let address = format!("{}:{}",
			  configuration.application.host, configuration.application.port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
