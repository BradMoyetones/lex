mod core;
mod api;
mod storage;

use axum::{routing::{post, get, put, delete}, Router, Extension};
use std::net::SocketAddr;
use sqlx::postgres::PgPoolOptions;
use dotenvy::dotenv;
use std::env;
use std::sync::Arc;
use core::registry::Registry;
use storage::postgres::PostgresAdapter;
use storage::StorageAdapter;

#[tokio::main]
async fn main() {
    dotenv().ok(); // Cargar el .env
    println!("--- LEX ENGINE: STARTING WITH DB ---");

    // 1. Inicializamos el Registry
    // Se cargan los módulos
    let registry = Arc::new(Registry::load_from_dir("modules"));

    // Creamos la conexión a la DB
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL no definida en .env");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("No se pudo conectar a la base de datos");

    // Usamos un Box dinámico o un tipo genérico para el adaptador
    let storage: Arc<dyn StorageAdapter> = Arc::new(PostgresAdapter { pool });

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
        .route("/api/modules/{name}", get(api::resource::list_items))
        // Rutas con ID
        .route("/api/modules/{name}/{id}", put(api::resource::update_item)) 
        .route("/api/modules/{name}/{id}", delete(api::resource::delete_item))
        // Ruta de descripción del módulo
        .route("/api/modules/{name}/describe", get(api::describe::describe_module))
        // 2. Compartimos tanto el pool como el registry
        .layer(Extension(registry))
        .layer(Extension(storage));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Lex escuchando en http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}