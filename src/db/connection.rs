use sea_orm::{Database, DatabaseConnection, DbErr};

/// Establishes a Postgres connection using `DATABASE_URL`.
pub async fn establish_connection(database_url: &str) -> Result<DatabaseConnection, DbErr> {
    Database::connect(database_url).await
}
