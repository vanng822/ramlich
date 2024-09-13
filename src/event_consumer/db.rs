use deadpool_postgres::Client;
use log::info;
use tokio_pg_mapper::FromTokioPostgresRow;

use crate::postres::DBPool;

use super::{errors::DBError, models::Request};

pub async fn add_request_event(request: Request) -> Result<String, DBError> {
    info!("add_request_event: {:#?}", request);
    let client: Client = DBPool::instance().get_client().await;
    let _stmt = include_str!("./sql/insert_request_event.sql");
    let _stmt = _stmt.replace("$table_fields", &Request::sql_table_fields());
    let statement = client.prepare(&_stmt).await.unwrap();
    let stored_request = client
        .query(
            &statement,
            &[
                &request.id,
                &request.url,
                &request.timestamp,
                &request.response_time,
            ],
        )
        .await;

    info!("add_request_event stored: {:#?}", stored_request);
    return Ok(request.id);
}
