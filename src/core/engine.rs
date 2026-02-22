use crate::core::contract::LexModule;
use crate::core::validator::{validate_data, ValidationError};
use serde_json::Value;

pub struct EngineResponse {
    pub success: bool,
    pub module: String,
    pub action_logs: Vec<String>,
    pub errors: Option<Vec<ValidationError>>,
}

pub fn execute_create(module: &LexModule, data: &Value) -> EngineResponse {
    let mut logs = Vec::new();

    // 1. Fase de Validación
    if let Err(errors) = validate_data(module, data) {
        return EngineResponse {
            success: false,
            module: module.metadata.name.clone(),
            action_logs: vec!["Validación fallida. Abortando.".to_string()],
            errors: Some(errors),
        };
    }

    logs.push("Validación exitosa".to_string());

    // 2. Fase de Hooks (beforeCreate)
    run_hooks(module, "beforeCreate", data, &mut logs);

    // 3. Fase de Persistencia (Aquí iría SQLX en el futuro)
    logs.push(format!("Guardando en tabla: {}", module.metadata.name));

    // 4. Fase de Hooks (afterCreate)
    run_hooks(module, "afterCreate", data, &mut logs);

    EngineResponse {
        success: true,
        module: module.metadata.name.clone(),
        action_logs: logs,
        errors: None,
    }
}

fn run_hooks(module: &LexModule, event: &str, _data: &Value, logs: &mut Vec<String>) {
    // Buscamos en el lex.json si hay hooks para este evento
    let hooks = module.spec.hooks.iter().filter(|h| h.on == event);

    for hook in hooks {
        // Ejecutamos la acción del hook
        // Por ahora, Lex "chismea" lo que está haciendo.
        logs.push(format!("🚀 [HOOK {}] Disparando acción: {}", event, hook.action));
    }
}