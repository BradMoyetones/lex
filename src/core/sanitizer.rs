pub fn is_safe_identifier(name: &str) -> bool {
    // Solo permitimos letras minúsculas, números y guiones bajos.
    // El nombre no puede empezar con un número.
    if name.is_empty() || name.len() > 63 { return false; }

    let mut chars = name.chars();
    
    // El primer caracter debe ser una letra
    if let Some(first) = chars.next() {
        if !first.is_ascii_lowercase() { return false; }
    }

    // El resto pueden ser letras, números o _
    chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
}