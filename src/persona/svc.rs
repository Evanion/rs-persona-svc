use serde::Serialize;
use sqlx::{SqlitePool};
use crate::common::app_state::AppState;

#[derive(Debug, Serialize)]
pub struct Persona {
  pub id: uuid::Uuid,
  pub identifiers: Vec<String>,
  pub created_at: chrono::DateTime<chrono::Utc>,
  pub created_now: Option<bool>,
  pub updated_at: chrono::DateTime<chrono::Utc>,
  pub parent: Option<Box<Persona>>,
  pub children: Vec<Persona>
}

pub async fn find_or_create(pool: &mut SqlitePool, identifier: &str, data: &AppState) -> Result<Persona, sqlx::Error> {

  // create uuidv5 from identifier
  let id = uuid::Uuid::new_v5(&data.config.namespace, identifier.as_bytes());
let created_at = chrono::Utc::now();
  let mut tx = pool.begin().await?;
  
  // find or create persona by id
  let persona = sqlx::query_as!(
    Persona,
    r#"
      INSERT INTO personas (id, created_at, updated_at)
      VALUES ($1, $2, $2)
      ON CONFLICT (id) DO NOTHING
      RETURNING id, created_at, updated_at
    "#,
    id,
    created_at
  );

  tx.commit().await?;

  persona.created_now = persona.created_at.eq(created_at);

  Ok(persona)
}