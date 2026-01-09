# Introducción a Web3 y Seguridad de Smart Contracts en Stellar

El crecimiento del ecosistema Web3 ha abierto nuevas oportunidades para construir servicios descentralizados, transparentes y sin intermediarios. Sin embargo, junto con estos beneficios surge un desafío inevitable: los errores en código desplegado en blockchain pueden tener consecuencias reales y permanentes.

Este artículo analiza:

* Qué es Web3 y qué aporta
* Cómo funciona la blockchain en Stellar
* Qué son los smart contracts en Stellar
* Por qué suelen ser vulnerables
* Un análisis detallado de una vulnerabilidad real de lógica incorrecta
* Cómo reproducirla y explotarla en Testnet
* Dónde encontrar el código
* Cómo continuar capacitándose mediante proyectos prácticos

---

## Qué es Web3

Web3 describe una transición hacia aplicaciones sin intermediarios, construidas sobre criptografía y blockchain.
Sus principios fundamentales son:

1. Propiedad nativa: los usuarios controlan sus activos digitales mediante claves privadas.
2. Transparencia y trazabilidad: la lógica vive en contratos desplegados públicamente.
3. Descentralización: los datos no dependen de un proveedor único.
4. Inmutabilidad: los cambios de estado o contratos no pueden revertirse arbitrariamente.

Este paradigma habilita nuevos modelos como pagos programables, DAOs, tokenización, infraestructura financiera abierta y economías colaborativas.

---

## Blockchain en Stellar

Stellar es una red blockchain diseñada para transferencias rápidas, comisiones bajas y escalabilidad.
Sus características centrales incluyen:

* Finalidad rápida de transacciones mediante Stellar Consensus Protocol (SCP)
* Tarifas mínimas y predecibles
* Enfoque en casos financieros reales
* Compatibilidad con contratos inteligentes escritos en Rust y compilados a WebAssembly

Stellar se ha consolidado como plataforma para pagos globales, stablecoins, y cada vez más para smart contracts orientados a seguridad y finanzas.

---

## Smart Contracts en Stellar

Los contratos inteligentes en Stellar se escriben en Rust y se compilan a WebAssembly (WASM).
Un contrato define reglas inmutables, tales como:

* Validar transacciones según parámetros definidos
* Aplicar condiciones automáticas
* Administrar balances y estado interno
* Proteger recursos mediante funciones, autorizaciones y control de almacenamiento

Una propiedad crítica es que la lógica desplegada es pública y permanente.
Cualquier error en esa lógica se convierte en un riesgo de seguridad explotable.

---

## Por qué los Smart Contracts son vulnerables

Los errores en contratos inteligentes no solo son posibles, sino frecuentes.
Las razones principales incluyen:

1. Código inmutable: una vez desplegado, no puede actualizarse fácilmente.
2. Suposiciones incorrectas: casos extremos o inesperados no cubiertos.
3. Errores de autorización: funciones críticas accesibles a cualquiera.
4. Mal manejo de condiciones lógicas: comparaciones incorrectas, ramas invertidas y booleanos ambiguos.
5. Escenario adversarial: cualquier operación abierta al público será sometida a análisis malicioso.

El resultado de un error es tangible: pérdida de fondos, corrupción de estado o ataques sistemáticos.

---

## Vulnerabilidad de Lógica Incorrecta: Caso de Estudio

A continuación presentamos un análisis concreto a partir del challenge “Lottery Logic Bug”.

### Descripción del Contrato

El contrato implementa una lotería donde:

* Un propietario inicializa un número secreto y un pozo de premios
* Los usuarios depositan fondos adicionales
* Cualquier jugador puede intentar adivinar el número
* Si acierta, debería ganar el premio

Sin embargo, la función que evalúa el resultado contiene una condición invertida:

```rust
if guess != secret {
    env.storage().persistent().set(&DataKey::Prize, &0);
    return true;
}
```

La intención era recompensar al usuario cuando el número es correcto.
La implementación real recompensa al usuario precisamente cuando se equivoca.

### Impacto del error

* Cualquier atacante puede ganar sin conocer el número secreto
* El pozo completo se drena en una transacción
* No requiere privilegios ni trucos complejos
* La explotación solo requiere enviar un valor incorrecto

Este error es un ejemplo clásico donde una sola comparación incorrecta destruye el modelo de seguridad completo.

---

## Reproducción del Ataque en Stellar Testnet

El challenge proporciona instrucciones claras:

1. Preparar entorno Rust + Stellar CLI
2. Compilar el contrato
3. Desplegarlo en testnet
4. Inicializar el pozo
5. Invocar la función `play` con un número incorrecto
6. Verificar que el premio se drena por completo

La observación principal es que, al invertir la lógica condicional, la rama de "error" se convierte en la ruta de explotación.

---

## Remediación

La corrección consiste en ajustar la condición lógica:

```rust
if guess == secret {
    env.storage().persistent().set(&DataKey::Prize, &0);
    return true;
}
```

Adicionalmente, se recomienda:

* Pruebas unitarias para cada rama de decisión
* Auditorías de pares
* Casos negativos explícitos
* Verificación de efectos sobre estado persistente

El mayor aprendizaje es que la semántica debe coincidir con las reglas de negocio, y eso no puede asumirse sin pruebas.

---

## Por qué este ejemplo es útil

Este challenge ilustra una realidad:
Los errores más críticos no provienen necesariamente de ataques avanzados, sino de detalles sutiles:

* Un operador invertido
* Un booleano mal interpretado
* Un flujo sin pruebas negativas

Comprender cómo opera la vulnerabilidad permite:

* Identificarla en otros contratos
* Evitar cometer el mismo error al desarrollar
* Evaluar el impacto sobre fondos reales
* Interiorizar prácticas de seguridad orientadas a blockchain

---

## Dónde encontrar el reto

Repositorio público:
[https://github.com/HackBalam/Seguridad-Smart-Contracts-Rust-Reto1.git](https://github.com/HackBalam/Seguridad-Smart-Contracts-Rust-Reto1.git)

Incluye:

* Contrato vulnerable
* Instrucciones de despliegue
* Código fix
* Tests sugeridos

---

## Cómo seguir perfeccionándote

Este challenge es solo el primer paso de una serie práctica enfocada en seguridad Web3:

* Auditorías en Stellar
* Validación de permisos y roles
* Manejo de storage seguro
* Evitar fugas de fondos
* Simulación de ataques reales
* Análisis de patrones de vulnerabilidad

El objetivo es aprender desde la implementación, no desde la teoría.

Quien completa los retos:

* Desarrolla criterio profesional
* Se entrena como auditor junior de smart contracts
* Construye portafolio técnico verificable
* Comprende realmente cómo se explota un contrato y cómo se previene

---

## Conclusión

La seguridad en Web3 no es opcional.
Las ramificaciones de un bug se traducen directamente en pérdida de capital, reputación y confianza.

Stellar ofrece una plataforma accesible, moderna y segura para construir contratos inteligentes, pero la responsabilidad final recae en quien escribe y despliega el código.

Aprender, auditar y romper contratos vulnerables bajo un entorno controlado es la manera más efectiva de comprender qué protege y qué expone a un proyecto Web3.
