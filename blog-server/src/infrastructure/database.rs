use sqlx::{PgPool, postgres::PgPoolOptions};
use tracing;

/// Создает пул подключений к базе данных PostgreSQL.
pub async fn create_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .min_connections(5)
        .acquire_timeout(std::time::Duration::from_secs(5))
        .connect(database_url)
        .await?;
    tracing::info!("connected to PostgreSQL");
    Ok(pool)
}

/// Запускает миграции базы данных.
pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::Error> {
    tracing::info!("running database migrations");
    sqlx::migrate!().run(pool).await?;
    tracing::info!("migrations completed");
    Ok(())
}
