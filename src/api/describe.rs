use axum::{extract::Path, Json, http::StatusCode, response::IntoResponse};
use std::fs;
use crate::core::LexModule;

pub async fn describe_module(
    Path(module_name): Path<String>,
) -> impl IntoResponse {
    let path = format!("modules/{}/lex.json", module_name);
    
    match fs::read_to_string(path) {
        Ok(data) => {
            // Validamos que el JSON sea un LexModule válido antes de enviarlo
            match serde_json::from_str::<LexModule>(&data) {
                Ok(module) => (StatusCode::OK, Json(module)).into_response(),
                Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error al parsear el contrato").into_response(),
            }
        },
        Err(_) => (StatusCode::NOT_FOUND, "Módulo no encontrado").into_response(),
    }
}