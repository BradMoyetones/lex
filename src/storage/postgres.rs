use crate::core::contract::LexModule;
use crate::storage::StorageAdapter;
use serde_json::Value;
use async_trait::async_trait;
use sqlx::PgPool;

pub struct PostgresAdapter {
    pub pool: PgPool,
}

#[async_trait]
impl StorageAdapter for PostgresAdapter {
    // Aquí pegamos la lógica de "CREATE SCHEMA" y "CREATE TABLE IF NOT EXISTS"
    // pero adaptada a retornar Result<(), String>
    async fn ensure_repository(&self, module: &LexModule) -> Result<(), String> {
        crate::storage::adapter::ensure_table(&self.pool, module).await.map_err(|e| e.to_string())
    }
    // Operaciones CRUD básicas
    async fn insert(&self, module: &LexModule, data: &Value) -> Result<(), String> {
        crate::storage::adapter::insert_data(&self.pool, module, data).await.map_err(|e| e.to_string())
    }
    async fn list(&self, module: &LexModule) -> Result<Vec<Value>, String> {
        crate::storage::adapter::list_data(&self.pool, module).await.map_err(|e| e.to_string())
    }
    async fn update(&self, module: &LexModule, id: String, data: &Value) -> Result<(), String> {
        crate::storage::adapter::update_data(&self.pool, module, id, data).await.map_err(|e| e.to_string())
    }
    async fn delete(&self, module: &LexModule, id: String) -> Result<(), String> {
        crate::storage::adapter::delete_data(&self.pool, module, id).await.map_err(|e| e.to_string())
    }
}