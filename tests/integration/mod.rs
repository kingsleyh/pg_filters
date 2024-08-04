use testcontainers_modules::postgres::Postgres;
use testcontainers_modules::testcontainers::ContainerAsync;
use testcontainers_modules::testcontainers::core::error;
use testcontainers_modules::testcontainers::runners::AsyncRunner;
use tokio::sync::OnceCell;
use tokio_postgres::{Client, NoTls};

pub mod integration_test;

pub async fn client() -> Client {
    get_client(get_connection_string().await).await
}

async fn get_connection_string() -> String {
    let container = get_container().await.as_ref().unwrap();
    let port = container.get_host_port_ipv4(5432).await.unwrap();
    let host = container.get_host().await.unwrap();
    format!("host={} user=postgres password=postgres port={}", host, port)
}

async fn get_client(connection_string: String) -> Client {
    let (client, connection) =
        tokio_postgres::connect(connection_string.as_str(), NoTls).await.unwrap();

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    client
}

pub async fn get_container() -> &'static error::Result<ContainerAsync<Postgres>> {
    static ONCE: OnceCell<error::Result<ContainerAsync<Postgres>>> = OnceCell::const_new();
    ONCE.get_or_init(|| async { Postgres::default().start().await }).await
}