# FitBalance

FitBalance es una aplicación de escritorio para el seguimiento de la alimentación, actividad física y balance calórico diario.

El proyecto está desarrollado íntegramente en Rust y está dividido en dos aplicaciones: una API REST encargada de la lógica de negocio y acceso a datos, y una aplicación de escritorio desarrollada con egui.

## 🚀 Características

- Registro e inicio de sesión de usuarios.
- Autenticación mediante JWT.
- Gestión de alimentos y calorías.
- Registro de alimentos consumidos.
- Registro de actividad física.
- Cálculo de calorías consumidas y gastadas.
- Seguimiento del balance calórico diario.
- Resumen semanal de actividad y alimentación.
- Gestión del perfil del usuario.
- Envío de códigos de verificación por correo electrónico.
- Persistencia de datos mediante PostgreSQL.

## 🏗️ Arquitectura

El proyecto está dividido en dos aplicaciones:

### API

`API_Calculadora_Calorias`

API REST desarrollada en Rust utilizando:

- Axum
- SQLx
- PostgreSQL
- JWT
- Argon2
- Tokio
- Resend

La API se encarga de la autenticación, gestión de usuarios, alimentos, actividad física y comunicación con la base de datos.

### Aplicación de escritorio

`Calculadora_Calorias`

Aplicación de escritorio desarrollada en Rust utilizando:

- egui
- eframe
- reqwest
- Serde

La aplicación proporciona la interfaz gráfica desde la que el usuario interactúa con FitBalance y se comunica con la API.

## 🛠️ Tecnologías

| Tecnología | Uso |
|---|---|
| Rust | Lenguaje principal |
| egui / eframe | Interfaz gráfica |
| Axum | API REST |
| SQLx | Acceso a base de datos |
| PostgreSQL | Base de datos |
| Tokio | Runtime asíncrono |
| JWT | Autenticación |
| Argon2 | Hashing de contraseñas |
| Serde | Serialización y deserialización |
| Resend | Envío de emails |

## 📁 Estructura del proyecto

```text
Pedro-FitBalance/
│
├── API_Calculadora_Calorias/
│   ├── src/
│   │   ├── alimentos/
│   │   ├── middleware/
│   │   ├── controllers.rs
│   │   ├── db.rs
│   │   ├── repository.rs
│   │   └── routes.rs
│   ├── .env.example
│   ├── Cargo.toml
│   └── Cargo.lock
│
├── Calculadora_Calorias/
│   ├── src/
│   │   ├── alimentos.rs
│   │   ├── index.rs
│   │   ├── login.rs
│   │   ├── perfil.rs
│   │   ├── register.rs
│   │   ├── resumen.rs
│   │   └── semanales.rs
│   ├── Cargo.toml
│   └── Cargo.lock
│
└── .gitignore
