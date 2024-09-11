use deadpool_postgres::{Client, Manager, ManagerConfig, Pool, RecyclingMethod};
use log::info;
use once_cell::sync::OnceCell;
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_postgres::NoTls;

use crate::{kafka::RequestEvent, postres};

use super::{errors::DBError, models::Request};

fn get_pool(
    db_port: u16,
    db_host: String,
    db_user: String,
    db_passwd: String,
    db_dbname: String,
) -> Pool {
    let mut pg_config = tokio_postgres::Config::new();
    pg_config.host(db_host);
    pg_config.port(db_port);
    pg_config.user(db_user);
    pg_config.password(db_passwd);
    pg_config.dbname(db_dbname);

    let mgr_config = ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    };
    let mgr = Manager::from_config(pg_config, NoTls, mgr_config);

    let pool = Pool::builder(mgr).max_size(16).build().unwrap();
    return pool;
}

pub struct DBPool {
    pool: Pool,
}

static INSTANCE: OnceCell<DBPool> = OnceCell::new();

impl DBPool {
    fn new(pool: Pool) -> Self {
        return Self { pool };
    }

    pub fn instance() -> &'static DBPool {
        return INSTANCE.get().unwrap();
    }

    pub async fn get_client(&self) -> Client {
        return self.pool.get().await.unwrap();
    }
    pub fn init(
        db_port: u16,
        db_host: String,
        db_user: String,
        db_passwd: String,
        db_dbname: String,
    ) -> &'static Self {
        let pool = get_pool(db_port, db_host, db_user, db_passwd, db_dbname);
        let db_pool = Self::new(pool);
        let _ = INSTANCE.set(db_pool);

        return Self::instance();
    }
}

pub async fn add_request_event(request: Request) -> Result<Request, DBError> {
    info!("add_request_event: {:#?}", request);
    let client: Client = DBPool::instance().get_client().await;
    let _stmt = include_str!("./sql/insert_request_event.sql");
    let _stmt = _stmt.replace("$table_fields", &Request::sql_table_fields());
    let statement = client.prepare(&_stmt).await.unwrap();
    return client
        .query(
            &statement,
            &[
                &request.id,
                &request.url,
                &request.timestamp.to_string(),
                &request.response_time.to_string(),
            ],
        )
        .await
        .iter()
        .map(|row| Request::from_row_ref(row.first().unwrap()).unwrap())
        .collect::<Vec<Request>>()
        .pop()
        .ok_or(DBError::NotFound);
}
