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
const DB_USER_ADMIN: &'static str = "admin";
const DB_PASS_ADMIN: &'static str = "admin-se3ret";
const DB_USER_RW: &'static str = "rw";
const DB_PASS_USER_RW: &'static str = "rw-se3ret";
const DB_USER_MIGRATOR: &'static str = "migrator";
const DB_PASS_USER_MIGRATOR: &'static str = "migrator-se3ret";
const DB_NAME: &'static str = "pg-test-db";

const INIT_SCRIPT: &'static str = r#"
-- roles
CREATE ROLE schema_owner NOLOGIN;
CREATE ROLE migrator LOGIN PASSWORD 'migrator-se3ret';
CREATE ROLE rw LOGIN PASSWORD 'rw-se3ret';

GRANT schema_owner TO migrator;
GRANT schema_owner TO rw;

ALTER ROLE migrator SET ROLE = schema_owner;

-- schema ownership
ALTER SCHEMA public OWNER TO schema_owner;

-- default grants
ALTER DEFAULT PRIVILEGES IN SCHEMA public
GRANT SELECT, INSERT, UPDATE, DELETE ON TABLES TO rw;
ALTER DEFAULT PRIVILEGES IN SCHEMA public
GRANT USAGE, SELECT, UPDATE ON SEQUENCES TO rw;
"#;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct TestDb {
    pub pg_container: ContainerAsync<PostgresContainer>,
    pub admin_pool: PgPool,
    pub rw_pool: PgPool,
}

impl TestDb {
    pub async fn new() -> Result<Self> {
        let pg_container = Self::container_setup().await?;
        let port = pg_container.get_host_port_ipv4(DB_PORT).await?;

        let migrator_pool = Self::get_migrator_pool(port).await?;
        sqlx::migrate!("./migrations")
            .run(&migrator_pool)
            .await
            .expect("migrations failed");

        let admin_pool = Self::get_admin_pool(port).await?;
        let rw_pool = Self::get_rw_pool(port).await?;

        Ok(Self {
            pg_container,
            admin_pool,
            rw_pool,
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
            .with_init_sql(INIT_SCRIPT.to_string().into_bytes())
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
        Self::pool(&admin_database_url).await
    }

    pub async fn get_rw_pool(port: u16) -> Result<PgPool> {
        let admin_database_url =
            format!("postgres://{DB_USER_RW}:{DB_PASS_USER_RW}@{DB_HOST}:{port}/{DB_NAME}");
        Self::pool(&admin_database_url).await
    }

    pub async fn get_migrator_pool(port: u16) -> Result<PgPool> {
        let admin_database_url = format!(
            "postgres://{DB_USER_MIGRATOR}:{DB_PASS_USER_MIGRATOR}@{DB_HOST}:{port}/{DB_NAME}"
        );
        Self::pool(&admin_database_url).await
    }

    #[inline]
    async fn pool(url: &str) -> Result<PgPool> {
        let pool = PgPoolOptions::new()
            .max_connections(PG_MAX_CONNECTIONS)
            .connect(&url)
            .await?;
        Ok(pool)
    }
}
