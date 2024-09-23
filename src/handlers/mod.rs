extern crate amlich;
extern crate vncalendar;

mod converters;
use converters::date_to_response;
mod today;
pub use today::today_route;

mod lunar;
pub use lunar::lunar_route;

mod dates;
pub use dates::get_month_route;

use utoipa::OpenApi;

pub mod middleware;

pub mod amlich_com_proxy;

use crate::{
    models::VNDate,
    responses::{VNDateResponse, YearDatesResponse, YearMonthDatesResponse},
};

#[derive(OpenApi)]
#[openapi(info(description = "Solar to Lunar date converter"))]
#[openapi(
    paths(
        today::today_route,
        lunar::lunar_route,
        dates::get_month_route,
        amlich_com_proxy::amlich_com_proxy
    ),
    components(schemas(
        VNDateResponse,
        YearDatesResponse,
        YearMonthDatesResponse,
        VNDate,
        amlich_com_proxy::AmLichCalendarResult,
        amlich_com_proxy::AmLichCalendar,
    ),)
)]
pub struct ApiDoc;
