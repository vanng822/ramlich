use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;
use uuid::Uuid;

#[derive(Deserialize, PostgresMapper, Serialize, Debug)]
#[pg_mapper(table = "request_event")]
pub struct Request {
    pub id: Uuid,
    pub url: String,
    pub requested_at: DateTime<Utc>,
    pub response_time: i64,
    pub status_code: i32,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}
