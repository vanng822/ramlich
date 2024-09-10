use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;
use tokio_postgres::types::ToSql;

#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "request_event")]
pub struct Request {
    pub id: String,
    pub url: String,
    pub timestamp: i64,
    pub response_time: i32,
}
