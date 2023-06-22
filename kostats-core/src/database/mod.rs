use sqlx::{Pool, Postgres};

pub struct Database {
    pool: Pool<Postgres>,
}

impl Database {
    pub async fn new(path: String) -> anyhow::Result<Database> {
        Ok(Database {
            pool: sqlx::PgPool::connect(&path).await?,
        })
    }
}

pub mod player;
