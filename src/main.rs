mod common {
  #[path = "app-state.rs"]
  pub mod app_state;
  pub mod env;
}

mod persona {
  pub mod svc;
  pub mod routes;
}


use actix_web::{HttpServer, App, web};
use sqlx::sqlite::SqlitePoolOptions;
use crate::persona::routes::{find_or_create};
use crate::common::app_state::AppState;
use crate::common::env::Config;
use std::sync::Mutex;
use dotenv::dotenv;


#[actix_web::main]
pub async fn main() -> std::io::Result<()>{
  dotenv().ok();

  let config = Config::from_env();
  let db = SqlitePoolOptions::new()
    .connect(&config.database_url)
    .await
    .unwrap();

    sqlx::migrate!().run(&db).await;

  println!("Starting server at http://{}:{}", &config.host, &config.port);

  HttpServer::new(move || {
    // The scope for all post services
    App::new()
      .app_data(web::Data::new(AppState { 
        config: Config::from_env(),
        db: Mutex::new(db.clone()) 
      }))
      .service(
        web::scope("/v1")
          .service(web::resource("/persona")
            .route(web::post()
              .to(find_or_create)))
      )
  })
  .bind((Config::from_env().host.to_string(), Config::from_env().port))?
  .run()
  .await

}