use deadpool_postgres::Client;
use log::info;
use tokio_pg_mapper::FromTokioPostgresRow;
use uuid::Uuid;

use crate::{kafka::RequestEvent, postres::DBPool};

use super::{errors::DBError, models::Request};

macro_rules! prepare_statement_query {
    ($sql_filename:expr) => {
        include_str!($sql_filename)
    };

    ($sql_filename:expr, $return_field_placeholder:expr, $return_field_names:expr) => {{
        let _stmt = prepare_statement_query!($sql_filename);
        _stmt.replace($return_field_placeholder, $return_field_names)
    }};
}

pub async fn add_request_event(request: RequestEvent) -> Result<Uuid, DBError> {
    info!("add_request_event: {:#?}", request);
    let client: Client = DBPool::instance().get_client().await;
    let _stmt = prepare_statement_query!(
        "./sql/insert_request_event.sql",
        "$table_fields",
        &Request::sql_table_fields()
    );

    let statement = client.prepare(&_stmt).await.unwrap();
    let stored_request = client
        .query(
            &statement,
            &[
                &request.id,
                &request.url,
                &request.requested_at,
                &request.response_time,
                &(request.status_code as i32),
            ],
        )
        .await;

    info!("add_request_event stored_request: {:#?}", stored_request);
    return Ok(request.id);
}

pub async fn get_request_event(id: Uuid) -> Result<Request, DBError> {
    info!("get_request_event: {:#?}", id);
    let client: Client = DBPool::instance().get_client().await;
    let _stmt = prepare_statement_query!("./sql/get_request_event.sql");
    let statement = client.prepare(&_stmt).await.unwrap();
    let request = client
        .query(&statement, &[&id])
        .await
        .unwrap()
        .get(0)
        .map(|row| Request::from_row_ref(row))
        .unwrap()
        .unwrap(); // TODO: map error

    return Ok(request);
}
