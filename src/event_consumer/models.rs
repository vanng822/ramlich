use std::time::SystemTime;

use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;
use uuid::Uuid;

#[derive(Deserialize, PostgresMapper, Serialize, Debug)]
#[pg_mapper(table = "request_event")]
pub struct Request {
    pub id: Uuid,
    pub url: String,
    pub timestamp: SystemTime,
    pub response_time: u32,
    pub created_at: Option<SystemTime>,
    pub updated_at: Option<SystemTime>,
}
