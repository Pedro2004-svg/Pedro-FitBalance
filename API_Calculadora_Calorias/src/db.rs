use std::time::Duration;

use sqlx::{postgres::PgPoolOptions, PgPool};


pub async fn connect(database_url: &str) -> Result<PgPool, sqlx::Error>{
    //Se crea la conexion a la BBDD, imprimiendo en consola un error si no conecta

    match PgPoolOptions::new()
    .max_connections(10)
    .acquire_timeout(Duration::from_secs(10))
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