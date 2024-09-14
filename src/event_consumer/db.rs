use std::{borrow::Borrow, error::Error};

use deadpool_postgres::Client;
use log::info;
use tokio_pg_mapper::FromTokioPostgresRow;
use uuid::Uuid;

use crate::postres::DBPool;

use super::{errors::DBError, models::Request};

pub async fn add_request_event(request: Request) -> Result<Uuid, DBError> {
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

    info!("add_request_event stored_request: {:#?}", stored_request);
    // testing select
    let saved_request = get_request_event(request.id).await.unwrap();
    info!("add_request_event saved_request: {:#?}", saved_request);

    return Ok(saved_request.id);
}

pub async fn get_request_event(id: Uuid) -> Result<Request, DBError> {
    info!("get_request_event: {:#?}", id);
    let client: Client = DBPool::instance().get_client().await;
    let _stmt = include_str!("./sql/get_request_event.sql");
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
