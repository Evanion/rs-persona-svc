
use std::{env};
use serde::{Serialize, Deserialize};
use uuid::{Uuid};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
  pub host: String,
  pub port: u16,
  pub database_url: String,
  pub namespace: Uuid,
}
 
impl Config {
  pub fn from_env() -> Self {
    let host = match env::var("HOST") {
      Ok(host) => host,
      Err(_) => "127.0.0.1".to_string()
    };
    
    let port = match env::var("PORT") {
      Ok(port) => port.parse::<u16>().unwrap(),
      Err(_) => 8080,
    };

    let namespace: Uuid = match env::var("NAMESPACE") {
      Ok(namespace) => Uuid::try_parse(&namespace).unwrap(),
      Err(_) => Uuid::new_v4(),
    };

    let database_url = match env::var("DATABASE_URL") {
      Ok(database_url) => database_url,
      Err(_) => panic!("DATABASE_URL not set")
    };

    Config {
      host,
      port,
      database_url,
      namespace,
    }
  }
}
