pub mod adapter; // Tu lógica de SQL actual
pub mod postgres; // La implementación para Postgres

use async_trait::async_trait;
use crate::core::contract::LexModule;
use serde_json::Value;

#[async_trait]
pub trait StorageAdapter: Send + Sync {
    // Asegura que el lugar donde guardamos los datos existe (Tabla, Colección, etc.)
    async fn ensure_repository(&self, module: &LexModule) -> Result<(), String>;
    async fn insert(&self, module: &LexModule, data: &Value) -> Result<(), String>;
    async fn list(&self, module: &LexModule) -> Result<Vec<Value>, String>;
    async fn update(&self, module: &LexModule, id: String, data: &Value) -> Result<(), String>;
    async fn delete(&self, module: &LexModule, id: String) -> Result<(), String>;
}