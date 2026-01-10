mod core;
mod api;
mod storage;

use axum::{routing::{post, get}, Router, Extension};
use std::net::SocketAddr;
use sqlx::postgres::PgPoolOptions;
use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok(); // Cargar el .env
    println!("--- LEX ENGINE: STARTING WITH DB ---");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL no definida en .env");
    
    // Creamos la conexión a la DB
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("No se pudo conectar a la base de datos");

    // Compartimos el 'pool' con todas las rutas usando Extension
    let app = Router::new()
        // Ruta index mostrando mensaje de bienvenida a la version de Lex 1.0.0 en JSON
        .route("/", get(|| async { 
            axum::Json(serde_json::json!({
                "message": "Bienvenido a Lex Engine",
                "status": "running",
                "version": "1.0.0"
            }))
        }))
        // Rutas sin ID
        .route("/api/modules/{name}", post(api::resource::create_item))
        .route("/api/modules/{name}", axum::routing::get(api::resource::list_items))
        // Rutas con ID
        .route("/api/modules/{name}/{id}", axum::routing::put(api::resource::update_item)) 
        .route("/api/modules/{name}/{id}", axum::routing::delete(api::resource::delete_item))
        // Ruta de descripción del módulo
        .route("/api/modules/{name}/describe", get(api::describe::describe_module))
        .layer(Extension(pool));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Lex escuchando en http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}