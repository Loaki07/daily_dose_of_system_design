use email_newsletter_z2p::configuration::get_configuration;
use email_newsletter_z2p::email_client::{
    self, EmailClient,
};
use email_newsletter_z2p::startup::run;
use email_newsletter_z2p::telemetry::{
    get_subscriber, init_subscriber,
};
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber(
        "zero2prod".into(),
        "info".into(),
        std::io::stdout,
    );
    init_subscriber(subscriber);

    let configuration = get_configuration()
        .expect("Failed to read configuration.");
    let connection_pool = PgPoolOptions::new()
        .connect_lazy_with(
            configuration.database.with_db(),
        );

    let sender_email = configuration
        .email_client
        .sender()
        .expect("Invalid sender email address.");
    let email_client = EmailClient::new(
        configuration.email_client.base_url,
        sender_email,
    );

    let address = format!(
        "{}:{}",
        configuration.application.host,
        configuration.application.port,
    );
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool, email_client)?.await?;
    Ok(())
}
