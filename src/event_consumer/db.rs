use confik::Configuration;
use deadpool_postgres::{Client, Pool};
use once_cell::sync::OnceCell;
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_postgres::NoTls;

use crate::{kafka::RequestEvent, postres};

use super::{errors::DBError, models::Request};

fn get_pool() -> Pool {
    let config = postres::config::Config::builder().try_build().unwrap();
    let pool = config.pg.create_pool(None, NoTls).unwrap();
    return pool;
}

struct DBPool {
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
    pub fn init() -> &'static Self {
        let pool = get_pool();
        let db_pool = Self::new(pool);
        INSTANCE.set(db_pool);

        return Self::instance();
    }
}

pub async fn add_request_event(request: Request) -> Result<Request, DBError> {
    return Ok(request);
    /*let client: Client = DBPool::instance().get_client().await;
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
        .await?
        .iter()
        .map(|row| Request::from_row_ref(row).unwrap())
        .collect::<Vec<Request>>()
        .pop()
        .ok_or(DBError::NotFound);*/
}
