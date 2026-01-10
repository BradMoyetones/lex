use serde::Serialize;
use serde_json::Value;
use crate::core::contract::{LexModule, FieldType};
use regex::Regex;

#[derive(Debug, Serialize)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
}

pub fn validate_data(module: &LexModule, data: &Value) -> Result<(), Vec<ValidationError>> {
    let mut errors = Vec::new();

    for field in &module.spec.fields {
        let field_value = data.get(&field.id);

        // 1. Validar Requeridos
        if field.required && (field_value.is_none() || field_value.unwrap().is_null()) {
            errors.push(ValidationError {
                field: field.id.clone(),
                message: "Este campo es requerido".to_string(),
            });
            continue;
        }

        // Si el valor existe, validamos su tipo y constraints
        if let Some(value) = field_value {
            if !value.is_null() {
                // 2. Validar Tipos de Datos
                match field.field_type {
                    FieldType::String => {
                        if !value.is_string() {
                            errors.push(ValidationError {
                                field: field.id.clone(),
                                message: "Debe ser un texto".to_string(),
                            });
                        } else if let Some(constraints) = &field.constraints {
                            // Validar Regex si existe
                            if let Some(regex_val) = constraints.get("regex").and_then(|v| v.as_str()) {
                                let re = Regex::new(regex_val).unwrap();
                                if !re.is_match(value.as_str().unwrap()) {
                                    errors.push(ValidationError {
                                        field: field.id.clone(),
                                        message: format!("No cumple con el formato requerido: {}", regex_val),
                                    });
                                }
                            }
                        }
                    },
                    FieldType::Number => {
                        if !value.is_number() {
                            errors.push(ValidationError {
                                field: field.id.clone(),
                                message: "Debe ser un número".to_string(),
                            });
                        } else if let Some(constraints) = &field.constraints {
                            // Validar Min si existe
                            if let Some(min_val) = constraints.get("min").and_then(|v| v.as_f64()) {
                                if value.as_f64().unwrap() < min_val {
                                    errors.push(ValidationError {
                                        field: field.id.clone(),
                                        message: format!("El valor mínimo es {}", min_val),
                                    });
                                }
                            }
                        }
                    },
                    _ => { /* Por ahora ignoramos otros tipos para mantener el core mínimo */ }
                }
            }
        }
    }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}