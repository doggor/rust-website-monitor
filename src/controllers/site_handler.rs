use actix_web::{web, error, get, post, put, delete, Error, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::services::storage::Storage;

#[derive(Deserialize, Serialize)]
pub struct AddSiteRequest {
    domain: String,
}

#[derive(Deserialize, Serialize)]
pub struct UpdateSiteRequest {
    domain: String,
    active: bool,
}

#[derive(Deserialize, Serialize)]
pub struct SuccessResponse {
    success: bool,
}

#[get("/sites")]
pub fn list_sites(storage: web::Data<Storage>) -> Result<HttpResponse, Error> {
    let result = storage.list_sites();

    match result {
        Ok(list) => Ok(HttpResponse::Ok().json(list)),
        Err(err) => {
            println!("Got error: {}", err);
            Err(error::ErrorInternalServerError("Internal Server Error"))
        }
    }
}

#[post("/sites")]
pub fn add_site(
    request: web::Json<AddSiteRequest>,
    storage: web::Data<Storage>,
) -> Result<HttpResponse, Error> {
    let result = storage.add_site(&request.domain);
    match result {
        Ok(()) => Ok(HttpResponse::Ok().json(SuccessResponse { success: true })),
        Err(err) => {
            println!("Got error: {}", err);
            Err(error::ErrorInternalServerError("Internal Server Error"))
        }
    }
}

#[put("/sites/{id}")]
pub fn update_site(path: web::Path<(u32,)>, request: web::Json<UpdateSiteRequest>, storage: web::Data<Storage>) -> Result<HttpResponse, Error> {
    let site_id = path.0;

    let result = storage.update_site(site_id, Some(&request.domain), None, Some(request.active));

    match result {
        Ok(()) => Ok(HttpResponse::Ok().json(SuccessResponse { success: true })),
        Err(err) => {
            println!("Got error: {}", err);
            Err(error::ErrorInternalServerError("Internal Server Error"))
        }
    }
}

#[delete("/sites/{id}")]
pub fn remove_site(path: web::Path<(u32,)>, storage: web::Data<Storage>) -> Result<HttpResponse, Error> {
    let site_id = path.0;

    let result = storage.remove_site(site_id);

    match result {
        Ok(()) => Ok(HttpResponse::Ok().json(SuccessResponse { success: true })),
        Err(err) => {
            println!("Got error: {}", err);
            Err(error::ErrorInternalServerError("Internal Server Error"))
        }
    }
}