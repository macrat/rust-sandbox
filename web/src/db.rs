use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions, SqliteRow};
use sqlx::Result;
use sqlx::Row;

pub struct Note {
    pub id: i64,
    pub text: String,
}

#[derive(Clone)]
pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn connect(path: &str) -> Result<Database> {
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(
                SqliteConnectOptions::new()
                    .filename(path)
                    .create_if_missing(true),
            )
            .await?;

        sqlx::query("CREATE TABLE IF NOT EXISTS notes (id INTEGER PRIMARY KEY, text TEXT)")
            .execute(&pool)
            .await?;

        Ok(Database { pool })
    }

    pub async fn get_all(&self) -> Result<Vec<Note>> {
        Ok(sqlx::query("SELECT id, text FROM notes")
            .map(|row: SqliteRow| Note {
                id: row.get(0),
                text: row.get(1),
            })
            .fetch_all(&self.pool)
            .await?)
    }

    pub async fn register(&self, text: &String) -> Result<()> {
        sqlx::query("INSERT INTO notes (text) VALUES (?)")
            .bind(text)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn remove(&self, id: i64) -> Result<()> {
        sqlx::query("DELETE FROM notes WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
