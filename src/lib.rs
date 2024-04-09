use sea_orm::{Database, DatabaseConnection};



pub async fn run(database_url: &str) -> DatabaseConnection {
    let database = Database::connect(database_url).await;
    database.unwrap()
}