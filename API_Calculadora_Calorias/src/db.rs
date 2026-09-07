use sqlx::{postgres::PgPoolOptions, PgPool};


pub async fn connect(database_url: &str) -> Result<PgPool, sqlx::Error>{
    PgPoolOptions::new()
    .max_connections(10)
    .connect(database_url)
    .await?;

    match PgPoolOptions::new()
    .connect(database_url)
    .await
    {
        Ok(pool) => {
            println!("CONEXIÓN OK");
            Ok(pool)
        }
        Err(e) => {
            println!("ERROR CONEXIÓN DB: {:?}", e);
            Err(e)
        }
    }
}