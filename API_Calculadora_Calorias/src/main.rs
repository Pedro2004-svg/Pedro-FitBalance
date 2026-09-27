mod alimentos;
mod controllers;
mod db;
mod middleware;
mod repository;
mod routes;

use dotenvy::dotenv;
use std::env;
use std::sync::Arc;
use crate::alimentos::conexion::conexion_alimentos;

#[derive(Clone)]
pub struct AppState {
    pub pool: sqlx::PgPool,
    pub jwt_secret: Arc<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")?;
    // VarError implementa std::error::Error, así que si falta la variable
    // el servidor no arranca (mejor eso que usar un secreto por defecto).
    let jwt_secret = env::var("JWT_SECRET")?;

    let pool = db::connect(&database_url).await?;
    
    conexion_alimentos(&pool).await?;
    let app = routes::routes(pool, Arc::new(jwt_secret));
    let port = std::env::var("PORT")
    .unwrap_or_else(|_| "3000".to_string())
    .parse::<u16>()
    .expect("PORT debe ser un numero valido");
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("Iniciando en el puerto: {}", port);
    axum::serve(listener, app).await?;

    Ok(())
}