use axum::{extract::Path, Json, http::StatusCode, response::IntoResponse, Extension};
use sqlx::PgPool;
use serde_json::Value;
use std::fs;
use crate::core::contract::LexModule;
use crate::core::engine::execute_create;
use crate::storage::adapter::{ensure_table, insert_data, list_data};

pub async fn create_item(
    Extension(pool): Extension<PgPool>,
    Path(module_name): Path<String>,
    Json(payload): Json<Value>,
) -> impl IntoResponse {
    // 1. Cargar módulo
    let path = format!("modules/{}/lex.json", module_name);
    let config_data = fs::read_to_string(path).unwrap();
    let module: LexModule = serde_json::from_str(&config_data).unwrap();

    // 2. Asegurar infraestructura (Tabla)
    if let Err(_) = ensure_table(&pool, &module).await {
        return (StatusCode::INTERNAL_SERVER_ERROR, "Error de sincronización").into_response();
    }

    // 3. Validación y Hooks (Cerebro)
    let response = execute_create(&module, &payload);

    if response.success {
        // 4. PERSISTENCIA REAL
        match insert_data(&pool, &module, &payload).await {
            Ok(_) => {
                let mut logs = response.action_logs;
                logs.push("Dato persistido en disco con éxito".to_string());
                (StatusCode::CREATED, Json(logs)).into_response()
            },
            Err(e) => {
                println!("Error al insertar: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Error al guardar datos").into_response()
            }
        }
    } else {
        (StatusCode::BAD_REQUEST, Json(response.errors)).into_response()
    }
}

pub async fn list_items(
    Extension(pool): Extension<PgPool>,
    Path(module_name): Path<String>,
) -> impl IntoResponse {
    // 1. Cargar módulo para validar que existe
    let path = format!("modules/{}/lex.json", module_name);
    let config_data = match fs::read_to_string(path) {
        Ok(data) => data,
        Err(_) => return (StatusCode::NOT_FOUND, "Módulo no encontrado").into_response(),
    };
    let module: LexModule = serde_json::from_str(&config_data).unwrap();

    // 2. Llamar al adaptador para leer la tabla
    match list_data(&pool, &module).await {
        Ok(items) => (StatusCode::OK, Json(items)).into_response(),
        Err(e) => {
            println!("Error al leer: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Error al recuperar datos").into_response()
        }
    }
}

pub async fn update_item(
    Extension(pool): Extension<PgPool>,
    Path((module_name, id)): Path<(String, String)>, 
    Json(payload): Json<Value>,
) -> impl IntoResponse {
    let path = format!("modules/{}/lex.json", module_name);
    let config_data = fs::read_to_string(path).unwrap();
    let module: LexModule = serde_json::from_str(&config_data).unwrap();

    // Aquí Lex podría volver a pasar el motor de validación antes de actualizar
    
    match crate::storage::adapter::update_data(&pool, &module, id, &payload).await {
        Ok(_) => (StatusCode::OK, "Registro actualizado correctamente").into_response(),
        Err(e) => {
            println!("Error al actualizar: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Error al actualizar").into_response()
        },
    }
}

pub async fn delete_item(
    Extension(pool): Extension<PgPool>,
    Path((module_name, id)): Path<(String, String)>,
) -> impl IntoResponse {
    let path = format!("modules/{}/lex.json", module_name);
    let config_data = fs::read_to_string(path).unwrap();
    let module: LexModule = serde_json::from_str(&config_data).unwrap();

    match crate::storage::adapter::delete_data(&pool, &module, id).await {
        Ok(_) => (StatusCode::NO_CONTENT).into_response(), // 204 No Content es estándar para DELETE
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error al borrar").into_response(),
    }
}