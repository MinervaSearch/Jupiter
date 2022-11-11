use actix_web::{web, HttpResponse, HttpRequest};
use serde::Serialize;

use crate::DbPool;
use crate::infra::logging;
use crate::model::link_model::NewLink;
use crate::{repository::link_repository, AppResult};

pub fn query_config(cfg: &mut web::ServiceConfig) {
    cfg.service(submit_page);
}

#[actix_web::post("/workers/scrape/submit")]
pub async fn submit_page(_db: web::Data<DbPool>, request: HttpRequest) -> AppResult<HttpResponse> {

    // Print json body from request
    logging::log_info(&format!("Received request from {}", request.peer_addr().unwrap().ip()));
/*
    logging::log_info(&format!("Received site: {}", site.url));

    let mut tx = db.begin().await.unwrap();
    link_repository::insert_link(&mut tx, site.url.clone()).await;

    logging::log_debug(&format!("Inserted site: {}", site.url));
*/
    let response: GenericOK = GenericOK {
        success: true
    };

    Ok(HttpResponse::Ok().json(response))

}

#[derive(Serialize)]
pub struct GenericOK {
    pub success: bool
}