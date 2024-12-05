use chrono::NaiveDateTime;
use deadpool::managed::Timeouts;
use deadpool::Runtime;
use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use std::str::FromStr;
use std::time::Duration;
use testcontainers_modules::postgres::Postgres;
use testcontainers_modules::testcontainers::runners::AsyncRunner;
use tokio_postgres::{Config, NoTls};
use uuid::Uuid;

pub mod integration_test;

const DB_POOL_MAX_OPEN: u64 = 10;
const DB_POOL_TIMEOUT_SECONDS: u64 = 10;

fn try_get_pool(db_url: String) -> eyre::Result<Pool> {
    let config = Config::from_str(&db_url).map_err(|e| eyre::eyre!(e))?;

    let manager_config = ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    };
    let manager = Manager::from_config(config, NoTls, manager_config);
    let pool = Pool::builder(manager)
        .max_size(DB_POOL_MAX_OPEN as usize)
        .runtime(Runtime::Tokio1)
        .timeouts(Timeouts {
            wait: Some(Duration::from_secs(DB_POOL_TIMEOUT_SECONDS)),
            create: Some(Duration::from_secs(DB_POOL_TIMEOUT_SECONDS)),
            recycle: Some(Duration::from_secs(DB_POOL_TIMEOUT_SECONDS)),
        })
        .build();
    pool.map_err(|e| eyre::eyre!(e))
}

async fn setup_data(pool: &Pool) -> eyre::Result<()> {
    let client = pool.get().await?;

    client
        .execute(
            "CREATE TABLE person (
                    id             SERIAL           PRIMARY KEY,
                    name           TEXT             NOT NULL,
                    nickname       VARCHAR(200)     NOT NULL,
                    age            INTEGER          NOT NULL,
                    capacity       DOUBLE PRECISION NOT NULL,
                    active         BOOLEAN          NOT NULL,
                    registration   TIMESTAMP        NOT NULL,
                    uuid           UUID             NOT NULL
                )",
            &[],
        )
        .await?;

    for i in 0..20 {
        let registration_date = NaiveDateTime::parse_from_str(
            &format!("2023-10-{:02} 12:00:00", (i + 1)),
            "%Y-%m-%d %H:%M:%S",
        )?;

        let uuid = Uuid::parse_str(&format!("550e8400-e29b-41d4-a716-44665544000{}", i % 10))?;

        client
            .execute(
                "INSERT INTO person (
                name, nickname, age, capacity, active, registration, uuid
            ) VALUES ($1, $2, $3, $4, $5, $6, $7)",
                &[
                    &format!("name{}", i),
                    &format!("nickname{}", i),
                    &(i),
                    &(i as f64),
                    &(i % 2 == 0),
                    &registration_date,
                    &uuid,
                ],
            )
            .await?;
    }
    Ok(())
}

pub async fn run_with_container<F, Fut>(test: F)
where
    F: FnOnce(Pool) -> Fut,
    Fut: std::future::Future<Output = ()>,
{
    // Start a new container for each test
    let container = Postgres::default()
        .start()
        .await
        .expect("Failed to start container");

    let port = container.get_host_port_ipv4(5432).await.unwrap();
    let host = container.get_host().await.unwrap();
    let db_url = format!("postgres://postgres:postgres@{}:{}/postgres", host, port);

    let pool = try_get_pool(db_url).expect("Unable to create pool");

    // Set up schema for this test instance
    setup_data(&pool).await.expect("Failed to setup schema");

    // Run the test
    test(pool.clone()).await;

    // Clean up container
    if let Err(err) = container.stop().await {
        eprintln!("Failed to stop container: {:?}", err);
    }
}
