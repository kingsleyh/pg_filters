use testcontainers_modules::postgres::Postgres;
use testcontainers_modules::testcontainers::ContainerAsync;
use testcontainers_modules::testcontainers::core::error;
use testcontainers_modules::testcontainers::runners::AsyncRunner;
use tokio::sync::OnceCell;
use tokio_postgres::{Client, NoTls};

pub mod integration_test;

async fn get_client() -> Client {
    let container = get_container().await.as_ref().unwrap();
    let port = container.get_host_port_ipv4(5432).await.unwrap();
    let host = container.get_host().await.unwrap();
    let connection_string = format!("host={} user=postgres password=postgres port={}", host, port);

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

pub async fn setup_data() -> &'static bool {
    static ONCE: OnceCell<bool> = OnceCell::const_new();
    ONCE.get_or_init(|| async { _setup_data().await }).await
}

async fn _setup_data() -> bool {
    println!("Setting up data");
    create_table().await;
    create_rows().await;
    true
}

async fn create_table() {
    let client = get_client().await;
    client
        .execute(
            "CREATE TABLE person (
                id        SERIAL           PRIMARY KEY,
                name      TEXT             NOT NULL,
                nickname  VARCHAR(200)     NOT NULL,
                age       INTEGER          NOT NULL,
                capacity  DOUBLE PRECISION NOT NULL,
                active    BOOLEAN          NOT NULL
            )",
            &[],
        )
        .await
        .unwrap();
}

async fn create_rows() {
    let client = get_client().await;
    for i in 0..20 {
        client
            .execute(
                "INSERT INTO person (name, nickname, age, capacity, active) VALUES ($1, $2, $3, $4, $5)",
                &[
                    &format!("name{}", i),
                    &format!("nickname{}", i),
                    &i,
                    &(i as f64),
                    &(i % 2 == 0),
                ],
            )
            .await
            .unwrap();
    }
}

