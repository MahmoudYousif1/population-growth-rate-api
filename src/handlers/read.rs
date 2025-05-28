use crate::app_state::model::AppState;
use crate::operations::crud_operations::read::read_country_records;
use crate::utils::models::{QueryType, ReadQuery};
use actix_web::{HttpResponse, Result, http::header::ContentType, web};

pub async fn read_handler(
    state: web::Data<AppState>,
    query: web::Json<ReadQuery>,
) -> Result<HttpResponse> {
    if let QueryType::Country { name, iso3 } = &query.query {
        if name.is_none() && iso3.is_none() {
            return Ok(HttpResponse::BadRequest()
                .content_type(ContentType::plaintext())
                .body("Either `name` or `iso3` must be provided"));
        }
    }

    match read_country_records(&state, &query.query) {
        Ok(records) => Ok(HttpResponse::Ok()
            .content_type(ContentType::json())
            .json(records)),
        Err(err_msg) => Ok(HttpResponse::BadRequest()
            .content_type(ContentType::plaintext())
            .body(err_msg)),
    }
}
