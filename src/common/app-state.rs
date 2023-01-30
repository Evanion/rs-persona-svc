use crate::common::env::Config;
use sqlx::sqlite::SqlitePool;
use std::sync::Mutex;

pub struct AppState {
  pub config: Config,
  pub db: Mutex<SqlitePool>,
}