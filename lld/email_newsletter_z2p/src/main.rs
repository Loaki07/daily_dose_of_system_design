use email_newsletter_z2p::configuration::get_configuration;
use email_newsletter_z2p::startup::run;
use env_logger::Env;
use sqlx::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(
        Env::default().default_filter_or("info"),
    )
    .init();

    let configuration = get_configuration()
        .expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(
        &configuration.database.connection_string(),
    )
    .await
    .expect("Faile to connect to Postgres");
    let address = format!(
        "127.0.0.1:{}",
        configuration.application_port
    );
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
