use sea_orm::{Database, DatabaseConnection, DbErr};

pub async fn init_database() -> Result<DatabaseConnection, DbErr> {
    let database_url = std::env::var("DATABASE_URL").unwrap();
    println!("database url: {}", database_url);
    let db = Database::connect(database_url).await?;
    Ok(db)
}