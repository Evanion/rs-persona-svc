use std::error::Error;

use crate::common::app_state::AppState;
use actix_web::{ web, HttpResponse, Responder};
use actix_web::web::Json;
use serde::{Deserialize};
use crate::persona::svc;

#[derive(Deserialize)]
pub struct GetPersonaDto {
  identifier: String,
}

pub async fn find_or_create(data: web::Data<AppState>, body: Json<GetPersonaDto>) -> Result<impl Responder, Box<dyn Error>> {
  let mut db_pool = data.db.lock().unwrap();
  let result = svc::find_or_create(& mut db_pool, &body.identifier.to_string(), &data).await?;
  Ok(HttpResponse::Ok().json(result))
}