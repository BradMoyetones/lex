use std::collections::HashMap;
use crate::core::contract::LexModule;
use std::fs;

pub struct Registry {
    // Se utiliza Arc para que sea seguro compartirlo entre hilos
    pub modules: HashMap<String, LexModule>,
}

impl Registry {
    pub fn load_from_dir(dir: &str) -> Self {
        let mut modules = HashMap::new();
        // Se itera la carpeta /modules y se cargan los lex.json
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path().join("lex.json");
                if path.exists() {
                    let content = fs::read_to_string(&path).expect("Error leyendo contrato");
                    let module: LexModule = serde_json::from_str(&content).expect("JSON inválido");
                    modules.insert(module.metadata.name.clone(), module);
                }
            }
        }
        println!("Lex Registry: {} módulos cargados.", modules.len());
        Self { modules }
    }

    pub fn get_module(&self, name: &str) -> Option<&LexModule> {
        self.modules.get(name)
    }
}