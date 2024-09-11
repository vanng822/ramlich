use deadpool_postgres::Client;
use log::info;
use tokio_pg_mapper::FromTokioPostgresRow;

use crate::postres::DBPool;

use super::{errors::DBError, models::Request};

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
