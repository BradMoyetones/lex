use sqlx::{PgPool, Executor, Row};
use crate::core::LexModule;
use crate::core::contract::FieldType;
use crate::core::sanitizer::is_safe_identifier;

pub async fn ensure_table(pool: &PgPool, module: &LexModule) -> Result<(), sqlx::Error> {
    let namespace = &module.metadata.namespace; // p.ej. "core"
    let table_name = &module.metadata.name;      // p.ej. "inventory"
    
    // --- EL ESCUDO DE SEGURIDAD (Evitamos inyección SQL) ---
    if !is_safe_identifier(namespace) || !is_safe_identifier(table_name) {
        return Err(sqlx::Error::Configuration(
            "Nombre de namespace o módulo inválido (caracteres no permitidos)".into()
        ));
    }

    // Validar también cada nombre de columna (field.id)
    for field in &module.spec.fields {
        if !is_safe_identifier(&field.id) {
            return Err(sqlx::Error::Configuration(
                format!("Nombre de campo '{}' inválido", field.id).into()
            ));
        }
    }
    // ------------------------------------------------

    // 1. Crear el SCHEMA si no existe
    let schema_query = format!("CREATE SCHEMA IF NOT EXISTS {}", namespace);
    pool.execute(schema_query.as_str()).await?;

    // 2. Construir la tabla dentro de ese SCHEMA (namespace.tabla)
    let full_table_path = format!("{}.{}", namespace, table_name);
    
    let mut query = format!(
        "CREATE TABLE IF NOT EXISTS {} (id UUID PRIMARY KEY DEFAULT gen_random_uuid()", 
        full_table_path
    );

    for field in &module.spec.fields {
        if field.id == "id" { continue; }

        let sql_type = match field.field_type {
            FieldType::String => "TEXT",
            FieldType::Number => "DOUBLE PRECISION",
            FieldType::Boolean => "BOOLEAN",
            FieldType::Datetime => "TIMESTAMPTZ",
            _ => "TEXT",
        };

        let nullability = if field.required { "NOT NULL" } else { "NULL" };
        query.push_str(&format!(", {} {} {}", field.id, sql_type, nullability));
    }

    query.push_str(");");

    println!("Lex sincronizando Namespace [{}]: {}", namespace, full_table_path);
    pool.execute(query.as_str()).await?;
    
    Ok(())
}

pub async fn insert_data(
    pool: &PgPool, 
    module: &LexModule, 
    data: &serde_json::Value
) -> Result<(), sqlx::Error> {
    let table_name = format!("{}.{}", module.metadata.namespace, module.metadata.name);
    let mut columns = Vec::new();
    let mut values_placeholders = Vec::new();
    
    // Filtramos los campos que vienen en el JSON y existen en el contrato
    for (_idx, field) in module.spec.fields.iter().enumerate() {
        if let Some(_) = data.get(&field.id) {
            columns.push(field.id.clone());
            // SQLx usa $1, $2, $3 para parámetros en Postgres
            values_placeholders.push(format!("${}", columns.len()));
        }
    }

    let query_str = format!(
        "INSERT INTO {} ({}) VALUES ({})",
        table_name,
        columns.join(", "),
        values_placeholders.join(", ")
    );

    let mut query = sqlx::query(&query_str);

    // Bind dinámico de valores
    for field in &module.spec.fields {
        if let Some(val) = data.get(&field.id) {
            match field.field_type {
                FieldType::String => {
                    query = query.bind(val.as_str().unwrap_or(""));
                },
                FieldType::Number => {
                    query = query.bind(val.as_f64().unwrap_or(0.0));
                },
                FieldType::Boolean => {
                    query = query.bind(val.as_bool().unwrap_or(false));
                },
                _ => { // Para otros tipos, convertimos a String y guardamos como TEXT, en un futuro se incluirán más tipos
                    query = query.bind(val.to_string());
                }
            }
        }
    }

    println!("Lex ejecutando persistencia: {}", query_str);
    query.execute(pool).await?;

    Ok(())
}

pub async fn list_data(
    pool: &PgPool,
    module: &LexModule,
) -> Result<Vec<serde_json::Value>, sqlx::Error> {
    let table_name = format!("{}.{}", module.metadata.namespace, module.metadata.name);
    let query_str = format!("SELECT * FROM {}", table_name);

    let rows = sqlx::query(&query_str)
        .fetch_all(pool)
        .await?;

    let mut results = Vec::new();

    for row in rows {
        let mut map = serde_json::Map::new();
        
        // 1. El ID siempre es un UUID en nuestra tabla
        let id: uuid::Uuid = row.try_get("id").unwrap_or_default();
        map.insert("id".to_string(), serde_json::Value::String(id.to_string()));

        // 2. Iteramos por los campos definidos en el contrato
        for field in &module.spec.fields {
            let field_id = &field.id;
            
            let val = match field.field_type {
                FieldType::String => {
                    let s: Option<String> = row.try_get(field_id.as_str()).ok();
                    serde_json::to_value(s).unwrap_or(serde_json::Value::Null)
                },
                FieldType::Number => {
                    let n: Option<f64> = row.try_get(field_id.as_str()).ok();
                    serde_json::to_value(n).unwrap_or(serde_json::Value::Null)
                },
                FieldType::Boolean => {
                    let b: Option<bool> = row.try_get(field_id.as_str()).ok();
                    serde_json::to_value(b).unwrap_or(serde_json::Value::Null)
                },
                _ => serde_json::Value::Null,
            };
            map.insert(field_id.clone(), val);
        }
        results.push(serde_json::Value::Object(map));
    }

    Ok(results)
}

pub async fn update_data(
    pool: &sqlx::PgPool,
    module: &LexModule,
    id: String,
    data: &serde_json::Value,
) -> Result<(), sqlx::Error> {
    let table_path = format!("{}.{}", module.metadata.namespace, module.metadata.name);
    let mut set_clauses = Vec::new();
    let mut param_index = 1;

    // Construimos las cláusulas SET solo para los campos que vienen en el JSON
    for field in &module.spec.fields {
        if data.get(&field.id).is_some() {
            set_clauses.push(format!("{} = ${}", field.id, param_index));
            param_index += 1;
        }
    }

    let query_str = format!(
        "UPDATE {} SET {} WHERE id = ${}",
        table_path,
        set_clauses.join(", "),
        param_index
    );

    let mut query = sqlx::query(&query_str);

    // Bind de valores dinámicos
    for field in &module.spec.fields {
        if let Some(val) = data.get(&field.id) {
            match field.field_type {
                FieldType::String => { query = query.bind(val.as_str().unwrap_or("")); },
                FieldType::Number => { query = query.bind(val.as_f64().unwrap_or(0.0)); },
                FieldType::Boolean => { query = query.bind(val.as_bool().unwrap_or(false)); },
                _ => { query = query.bind(val.to_string()); } // Otros tipos como TEXT, mismo comportamiento que en insert
            }
        }
    }

    // El último parámetro es el ID para el WHERE
    let uuid_id = uuid::Uuid::parse_str(&id).unwrap_or_default();
    query.bind(uuid_id).execute(pool).await?;

    Ok(())
}

pub async fn delete_data(
    pool: &sqlx::PgPool,
    module: &LexModule,
    id: String,
) -> Result<(), sqlx::Error> {
    let table_path = format!("{}.{}", module.metadata.namespace, module.metadata.name);
    let query_str = format!("DELETE FROM {} WHERE id = $1", table_path);
    
    let uuid_id = uuid::Uuid::parse_str(&id).unwrap_or_default();
    sqlx::query(&query_str)
        .bind(uuid_id)
        .execute(pool)
        .await?;

    Ok(())
}