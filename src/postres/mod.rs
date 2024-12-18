use deadpool_postgres::{Client, Manager, ManagerConfig, Pool, RecyclingMethod};
use log::info;
use once_cell::sync::OnceCell;
use tokio_postgres::NoTls;

pub struct DBPool {
    pool: Pool,
}

static INSTANCE: OnceCell<DBPool> = OnceCell::new();

impl DBPool {
    fn new(pool: Pool) -> Self {
        Self { pool }
    }

    fn get_pool(db_port: u16, db_host: String, db_user: String, db_dbname: String) -> Pool {
        let mut pg_config = tokio_postgres::Config::new();
        pg_config.host(db_host);
        pg_config.port(db_port);
        pg_config.user(db_user);
        pg_config.dbname(db_dbname);

        let mgr_config = ManagerConfig {
            recycling_method: RecyclingMethod::Fast,
        };
        let mgr = Manager::from_config(pg_config, NoTls, mgr_config);

        Pool::builder(mgr)
            .max_size(16)
            .build()
            .expect("db pool created")
    }

    pub fn instance() -> &'static Self {
        INSTANCE.get().expect("Instance of DBPool")
    }

    pub async fn get_client(&self) -> Client {
        self.pool.get().await.expect("Client from db pool")
    }

    pub fn init(
        db_port: u16,
        db_host: String,
        db_user: String,
        db_dbname: String,
    ) -> &'static Self {
        info!("{}, {}, {}, {}", db_port, db_host, db_user, db_dbname);
        let pool = Self::get_pool(db_port, db_host, db_user, db_dbname);
        let db_pool = Self::new(pool);
        let _ = INSTANCE.set(db_pool);

        Self::instance()
    }
}
