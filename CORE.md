# Lex: La Anatomía del Sistema (Especificación Técnica)

Este documento detalla el funcionamiento interno de **Lex**, el flujo de datos desde la definición hasta la ejecución, y las decisiones tecnológicas que garantizan su supervivencia a largo plazo.

---

## 1. El Blueprint: La Ley (`lex.json`)

En Lex, el código no dicta la lógica; el **Contrato** lo hace. El archivo `lex.json` es la fuente de verdad única.

### Ejemplo de Referencia: Módulo de Inventario

```json
{
  "$schema": "https://lex.engine/v1/schema.json",
  "version": "1.0.0",
  "kind": "Module",
  "metadata": {
    "name": "inventory",
    "namespace": "core"
  },
  "spec": {
    "fields": [
      {
        "id": "sku",
        "type": "string",
        "required": true,
        "constraints": { "regex": "^[A-Z0-9-]{8,12}$" }
      },
      {
        "id": "stock",
        "type": "number",
        "default": 0,
        "constraints": { "min": 0 }
      }
    ],
    "hooks": [
      {
        "on": "beforeCreate",
        "action": "validate_inventory_limits"
      }
    ]
  }
}

```

---

## 2. El Ciclo de Vida del Dato

El motor procesa la definición a través de cinco etapas críticas:

1. **La Ley:** El usuario describe el inventario. Define que el `SKU` debe seguir un patrón y que el `stock` nunca puede ser negativo. No hay espacio para la ambigüedad.
2. **El Motor de Acero (Rust Core):** El motor digiere el JSON. Valida la estructura y crea representaciones en memoria. Rust asegura que nada que viole este contrato pueda entrar al sistema.
3. **Persistencia Transparente:** Lex mapea el contrato a la base de datos de forma agnóstica. Crea tablas, índices y restricciones físicas (SQL) o esquemas (NoSQL) automáticamente.
4. **El Espejo (`/describe`):** El motor expone su alma. A través de este endpoint, el sistema informa a cualquier consumidor exactamente qué campos, validaciones y capacidades posee.
5. **Creatividad Explosiva:** El ecosistema consume el espejo. UIs que se autogeneran, SDKs listos para usar e integraciones que reaccionan a eventos sin configuración manual.

---

## 3. El Motor de Acero: ¿Por qué Rust?

Para que Lex sea un estándar eterno, el lenguaje de implementación debe ser impecable. Rust fue elegido por tres razones innegociables:

* **Integridad de Memoria:** A diferencia de C++ o JS, Rust garantiza que no existan fugas de memoria o punteros nulos. El motor es sólido como una roca.
* **Seguridad en el Diseño:** El sistema de tipos de Rust nos obliga a manejar cada posible error en tiempo de compilación. Si el motor compila, el contrato es seguro.
* **Abstracciones de Coste Cero:** Podemos escribir código altamente expresivo para validar JSONs complejos sin sacrificar ni un milisegundo de rendimiento.

---

## 4. Arquitectura de Introspección

El poder de Lex reside en su capacidad de **verse a sí mismo**. El endpoint `/describe` no es solo documentación; es un **motor de descubrimiento** que permite:

| Beneficio | Descripción |
| --- | --- |
| **Zero-Config UI** | El Frontend lee los campos y renderiza el formulario correcto instantáneamente. |
| **Type-Safe SDKs** | Generación automática de tipos para TS, Python o Go basada en el contrato actual. |
| **Validación Dual** | La misma lógica de validación definida en el JSON se aplica en el cliente (UX) y en el motor (Seguridad). |
