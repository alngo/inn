use std::str::FromStr;

use anyhow::Context;
use async_trait::async_trait;
use thiserror::Error;

use crate::{application::OwnerRepository, domain::Owner};

#[derive(Debug, Clone)]
pub struct Sqlite {
    pool: sqlx::SqlitePool,
}

impl Sqlite {
    pub async fn new(path: &str) -> anyhow::Result<Sqlite> {
        let pool = sqlx::SqlitePool::connect_with(
            sqlx::sqlite::SqliteConnectOptions::from_str(path)
                .with_context(|| format!("invalid database path {}", path))?
                .pragma("foreign_keys", "ON"),
        )
        .await
        .with_context(|| format!("failed to open database at {}", path))?;

        Ok(Sqlite { pool })
    }
}

// #[derive(Debug, Clone, PartialEq, Eq, Error)]
// pub enum SqliteError {
//     #[error("failed to start SQLite transaction: {reason}")]
//     FailToStartSQLiteTx { reason : String },
//     #[error("failed to commit SQLite transaction: {reason}")]
//     FailToCommitSQLiteTx { reason : String },
// }

#[async_trait(?Send)]
impl OwnerRepository for Sqlite {
    async fn create_owner(
        &self,
        _req: &crate::application::create_owner::Request,
    ) -> Result<Owner, anyhow::Error> {
        let tx = self.pool.begin().await?;

        //        let id = uuid::Uuid::now_v7();
        //        let id_as_string = id.to_string();
        //        let query = sqlx::query!(
        //            "INSERT INTO authors (id, name) VALUES ($1, $2)",
        //            id_as_string,
        //            name
        //        );
        //        tx.execute(query).await?;

        tx.commit().await?;
        todo!()
    }
}
