---
theme: seriph
background: https://images.unsplash.com/photo-1633164962363-58fd218f7882?q=80&w=1080&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D
class: text-center
highlighter: shiki
lineNumbers: true
drawings:
  enabled: true
---

# LEX
### El Motor Declarativo Eterno
Inspirado en la arquitectura de Minecraft.

---

# ¿Por qué Lex?
El software actual es frágil. Las implementaciones mueren, los contratos se rompen.

- **Problema:** Lógica de negocio enterrada en código imperativo.
- **Solución:** Un sustrato agnóstico que describe, no decide.
- **Inspiración:** Un mundo hecho de bloques (módulos) con reglas universales.

---

# El Manifiesto del Núcleo Mínimo
> "Construir lo pequeño para permitir lo inmenso."

1. **El Motor describe; no decide.**
2. **Contrato único y estable (v1 forever).**
3. **Datos antes que código.**
4. **Pequeño núcleo, ecosistema gigante.**

---

# La Unidad de Verdad: `lex.json`
Todo sistema en Lex nace de una descripción.

```json
{
  "name": "inventory",
  "fields": [
    { "id": "sku", "type": "string", "required": true },
    { "id": "stock", "type": "number", "default": 0 }
  ]
}

```

---

# El Espejo: `/describe`

Cualquier cliente (React, Flutter, CLI) puede auto-generarse.

* **Introspección total:** El motor expone su propia alma.
* **SDKs automáticos:** Cero fricción para el desarrollador.
* **UIs Dinámicas:** La interfaz es una consecuencia del contrato.

---

## 2. Diagrama de Flujo UX (Mapa Mental)

Este es el flujo que describiremos para que una IA (o tú mismo en un canvas) lo dibuje. Representa cómo Lex transforma una idea en una realidad funcional:

**Flujo: Del Concepto a la Ejecución**

1.  **CAPA DE DISEÑO (Arquitecto)**
    * Entrada: Definición de necesidades.
    * Acción: Escritura de `lex.json` (Blueprint).
    * Herramienta: Lex Builder (Tu Front) o VS Code.

2.  **CAPA DE PROCESAMIENTO (Lex Core - Rust)**
    * Acción: Ingesta del JSON.
    * Validación: El motor verifica la integridad del contrato.
    * Efecto: Se "congela" el contrato. El motor prepara la persistencia (DB) y los hooks.

---

## 2. Diagrama de Flujo UX (Mapa Mental)

3.  **CAPA DE INTROSPECCIÓN (El Puente)**
    * Endpoint: `/modules/inventory/describe`.
    * Salida: Metadatos completos (Qué campos hay, qué reglas existen, qué eventos se disparan).

4.  **CAPA DE CONSUMO (Ecosistema)**
    * **UI Renderer:** Lee el `/describe` y dibuja formularios y tablas automáticamente.
    * **SDK Generator:** Crea librerías en TS/Python para hablar con el motor.
    * **Integraciones:** Otros módulos de Lex escuchan eventos (hooks) y reaccionan.

---