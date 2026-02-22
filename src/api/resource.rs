use axum::{extract::Path, Json, http::StatusCode, response::IntoResponse, Extension};
use serde_json::Value;
use crate::core::engine::execute_create;
use std::sync::Arc;
use crate::core::registry::Registry;
use crate::storage::StorageAdapter;

pub async fn create_item(
    Extension(registry): Extension<Arc<Registry>>,
    Extension(storage): Extension<Arc<dyn StorageAdapter>>, // Se desacopla de Postgres es decir se pueden construir otros adaptadores
    Path(module_name): Path<String>,
    Json(payload): Json<Value>,
) -> impl IntoResponse {
    // 1. Buscar módulo en memoria
    let module = match registry.modules.get(&module_name) {
        Some(m) => m,
        None => return (StatusCode::NOT_FOUND, "Módulo no encontrado").into_response(),
    };

    // 2. Asegurar que la tabla existe (vía el adaptador)
    if let Err(e) = storage.ensure_repository(module).await {
        return (StatusCode::INTERNAL_SERVER_ERROR, e).into_response();
    }

    // 3. Ejecutar cerebro de Lex (Validación y Hooks)
    let response = execute_create(module, &payload);

    if response.success {
        // 4. Guardar usando el adaptador (No importa si es Postgres o Mongo)
        match storage.insert(module, &payload).await {
            Ok(_) => (StatusCode::CREATED, "Dato guardado con éxito").into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
        }
    } else {
        (StatusCode::BAD_REQUEST, Json(response.errors)).into_response()
    }
}

pub async fn list_items(
    Extension(registry): Extension<Arc<Registry>>,
    Extension(storage): Extension<Arc<dyn StorageAdapter>>,
    Path(module_name): Path<String>,
) -> impl IntoResponse {
    // 1. Cargar módulo para validar que existe
    let module = match registry.modules.get(&module_name) {
        Some(m) => m,
        None => return (StatusCode::NOT_FOUND, "Módulo no encontrado").into_response(),
    };

    // 2. Llamar al adaptador para leer la tabla
    match storage.list(module).await {
        Ok(items) => (StatusCode::OK, Json(items)).into_response(),
        Err(e) => {
            println!("Error al leer: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Error al recuperar datos").into_response()
        }
    }
}

pub async fn update_item(
    Extension(registry): Extension<Arc<Registry>>,
    Extension(storage): Extension<Arc<dyn StorageAdapter>>,
    Path((module_name, id)): Path<(String, String)>, 
    Json(payload): Json<Value>,
) -> impl IntoResponse {
    let module = match registry.modules.get(&module_name) {
        Some(m) => m,
        None => return (StatusCode::NOT_FOUND, "Módulo no encontrado").into_response(),
    };

    match storage.update(module, id, &payload).await {
        Ok(_) => (StatusCode::OK, "Registro actualizado correctamente").into_response(),
        Err(e) => {
            println!("Error al actualizar: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Error al actualizar").into_response()
        },
    }
}

pub async fn delete_item(
    Extension(registry): Extension<Arc<Registry>>,
    Extension(storage): Extension<Arc<dyn StorageAdapter>>,
    Path((module_name, id)): Path<(String, String)>,
) -> impl IntoResponse {
    let module = match registry.modules.get(&module_name) {
        Some(m) => m,
        None => return (StatusCode::NOT_FOUND, "Módulo no encontrado").into_response(),
    };

    match storage.delete(module, id).await {
        Ok(_) => (StatusCode::NO_CONTENT).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error al borrar").into_response(),
    }
}