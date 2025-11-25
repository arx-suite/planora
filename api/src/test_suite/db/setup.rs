#![allow(dead_code)]

use sqlx::{PgPool, postgres::PgPoolOptions};
use testcontainers_modules::{
    postgres::Postgres as PostgresContainer,
    testcontainers::{ContainerAsync, ImageExt, runners::AsyncRunner},
};

const TESTCONTAINER_TAG: &'static str = "17.5";
const TESTCONTAINER_CONTAINER_NAME: &'static str = "arx-testhouse";
const PG_MAX_CONNECTIONS: u32 = 3;
const DB_HOST: &'static str = "localhost";
const DB_PORT: u16 = 5432;
const DB_USER_ADMIN: &'static str = "pg-test-admin";
const DB_PASS_ADMIN: &'static str = "pg-test-admin-pass";
const DB_USER_USER1: &'static str = "pg-test-user1";
const DB_PASS_USER1: &'static str = "pg-test-user1-pass";
const DB_NAME: &'static str = "pg-test-db";

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct TestDb {
    pg_container: ContainerAsync<PostgresContainer>,
    admin_pool: PgPool,
    // user1_pool: PgPool,
}

impl TestDb {
    pub async fn new() -> Result<Self> {
        let pg_container = Self::container_setup().await?;
        let port = pg_container.get_host_port_ipv4(DB_PORT).await?;

        let admin_pool = Self::get_admin_pool(port).await?;
        sqlx::migrate!("./migrations")
            .run(&admin_pool)
            .await
            .expect("migrations failed");

        // let user1_pool = Self::get_user1_pool(port).await?;

        Ok(Self {
            pg_container,
            admin_pool,
        })
    }

    pub async fn teardown(self) -> Result<()> {
        self.admin_pool.close().await;

        if self.pg_container.is_running().await? {
            self.pg_container.stop().await?;
        }

        Ok(())
    }

    async fn container_setup() -> Result<ContainerAsync<PostgresContainer>> {
        let container = PostgresContainer::default()
            .with_user(DB_USER_ADMIN)
            .with_password(DB_PASS_ADMIN)
            .with_db_name(DB_NAME)
            .with_tag(TESTCONTAINER_TAG)
            .with_container_name(TESTCONTAINER_CONTAINER_NAME)
            .start()
            .await?;

        Ok(container)
    }

    pub async fn get_admin_pool(port: u16) -> Result<PgPool> {
        let admin_database_url =
            format!("postgres://{DB_USER_ADMIN}:{DB_PASS_ADMIN}@{DB_HOST}:{port}/{DB_NAME}");

        let pool = PgPoolOptions::new()
            .max_connections(PG_MAX_CONNECTIONS)
            .connect(&admin_database_url)
            .await?;

        Ok(pool)
    }

    pub async fn get_user1_pool(port: u16) -> Result<PgPool> {
        let admin_database_url =
            format!("postgres://{DB_USER_USER1}:{DB_PASS_USER1}@{DB_HOST}:{port}/{DB_NAME}");

        let pool = PgPoolOptions::new()
            .max_connections(PG_MAX_CONNECTIONS)
            .connect(&admin_database_url)
            .await?;

        Ok(pool)
    }
}
