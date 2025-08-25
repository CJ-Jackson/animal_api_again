use crate::animal::object::{AnimalAddUpdateObject, AnimalObject};
use crate::common::context::{Context, ContextError, FromContext};
use crate::common::db::SqliteClient;
use error_stack::{Report, ResultExt};
use rusqlite::named_params;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AnimalRepositoryError {
    #[error("Query error")]
    QueryError,
    #[error("Row Value error")]
    RowValueError,
    #[error("Lock error")]
    LockError,
    #[error("Not found error")]
    NotFoundError,
}

pub struct AnimalRepository {
    sqlite_client: SqliteClient,
}

impl AnimalRepository {
    pub fn new(sqlite_client: SqliteClient) -> Self {
        Self { sqlite_client }
    }

    pub fn add_animal(
        &self,
        object: &AnimalAddUpdateObject,
    ) -> Result<(), Report<AnimalRepositoryError>> {
        let conn = self
            .sqlite_client
            .get_conn()
            .lock()
            .map_err(|_| AnimalRepositoryError::LockError)?;

        conn.execute(
            include_str!("_sql/add_animal.sql"),
            named_params! {
                ":species": object.species,
                ":description": object.description,
            },
        )
        .change_context(AnimalRepositoryError::QueryError)?;

        Ok(())
    }

    pub fn fetch_all_animals(&self) -> Result<Box<[AnimalObject]>, Report<AnimalRepositoryError>> {
        let conn = self
            .sqlite_client
            .get_conn()
            .lock()
            .map_err(|_| AnimalRepositoryError::LockError)?;

        let mut stmt = conn
            .prepare(include_str!("_sql/fetch_all_animals.sql"))
            .change_context(AnimalRepositoryError::QueryError)?;

        let item_iter = stmt
            .query_map([], |row| {
                Ok(AnimalObject {
                    id: row.get("id")?,
                    species: row.get("species")?,
                    description: row.get("description")?,
                })
            })
            .change_context(AnimalRepositoryError::QueryError)?;

        let mut items = Vec::new();
        for item in item_iter {
            items.push(item.change_context(AnimalRepositoryError::RowValueError)?);
        }

        Ok(items.into())
    }

    pub fn fetch_animal_by_id(
        &self,
        id: i64,
    ) -> Result<AnimalObject, Report<AnimalRepositoryError>> {
        let conn = self
            .sqlite_client
            .get_conn()
            .lock()
            .map_err(|_| AnimalRepositoryError::LockError)?;

        let mut stmt = conn
            .prepare(include_str!("_sql/fetch_animal_by_id.sql"))
            .change_context(AnimalRepositoryError::QueryError)?;

        let item_iter = stmt.query_map(
            named_params! {
                ":id": id,
            },
            |row| {
                Ok(AnimalObject {
                    id: row.get("id")?,
                    species: row.get("species")?,
                    description: row.get("description")?,
                })
            },
        );

        let item = item_iter
            .change_context(AnimalRepositoryError::QueryError)?
            .next()
            .ok_or(AnimalRepositoryError::NotFoundError)?;

        Ok(item.change_context(AnimalRepositoryError::RowValueError)?)
    }

    pub fn update_animal(
        &self,
        object: &AnimalAddUpdateObject,
        id: i64,
    ) -> Result<(), Report<AnimalRepositoryError>> {
        let conn = self
            .sqlite_client
            .get_conn()
            .lock()
            .map_err(|_| AnimalRepositoryError::LockError)?;

        let _ = conn
            .execute(
                include_str!("_sql/update_animals.sql"),
                named_params! {
                    ":species": object.species,
                    ":description": object.description,
                    ":id": id,
                },
            )
            .change_context(AnimalRepositoryError::QueryError)?;

        Ok(())
    }
}

impl FromContext for AnimalRepository {
    async fn from_context(ctx: &'_ Context<'_>) -> Result<Self, Report<ContextError>> {
        Ok(Self::new(SqliteClient::from_context(ctx).await?))
    }
}
