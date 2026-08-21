# FitBalance

FitBalance es una aplicación de escritorio para el seguimiento de la alimentación y el balance calórico diario.

El proyecto está desarrollado íntegramente en **Rust** y está dividido en dos aplicaciones: una **API REST** encargada de la lógica de negocio y el acceso a datos, y una **aplicación de escritorio** desarrollada con `egui`.

## 🚀 Características

* Registro e inicio de sesión de usuarios.
* Autenticación mediante JWT.
* Gestión de alimentos y calorías.
* Registro de alimentos consumidos.
* Cálculo de calorías consumidas y gastadas.
* Seguimiento del balance calórico diario.
* Resumen semanal de alimentación.
* Gestión del perfil del usuario.
* Envío de códigos de verificación por correo electrónico.
* Persistencia de datos mediante PostgreSQL.

## 📧 Envío de correos

FitBalance utiliza [Resend](https://resend.com/) para el envío de códigos de verificación por correo electrónico.

La configuración actual está orientada al entorno de desarrollo y utiliza una dirección de correo autorizada. Para desplegar el sistema en producción y permitir el envío desde una dirección propia, es necesario configurar y verificar un dominio en Resend.

## 🏗️ Arquitectura

El proyecto está dividido en dos aplicaciones independientes:

### API

`API_Calculadora_Calorias`

API REST desarrollada en Rust utilizando:

* **Axum** — Framework web para la API REST.
* **SQLx** — Acceso asíncrono a la base de datos.
* **PostgreSQL** — Sistema de gestión de base de datos.
* **JWT** — Autenticación y autorización.
* **Argon2** — Hashing de contraseñas.
* **Tokio** — Runtime asíncrono.
* **Resend** — Envío de correos electrónicos.

La API se encarga de la autenticación, gestión de usuarios, gestión de alimentos, cálculo de calorías y comunicación con la base de datos.

### Aplicación de escritorio

`Calculadora_Calorias`

Aplicación de escritorio desarrollada en Rust utilizando:

* **egui / eframe** — Interfaz gráfica.
* **reqwest** — Comunicación HTTP con la API.
* **Serde** — Serialización y deserialización de datos.

La aplicación proporciona la interfaz gráfica desde la que el usuario interactúa con FitBalance y se comunica con la API REST.

## 🛠️ Tecnologías

| Tecnología    | Uso                             |
| ------------- | ------------------------------- |
| Rust          | Lenguaje principal              |
| egui / eframe | Interfaz gráfica                |
| Axum          | API REST                        |
| SQLx          | Acceso a base de datos          |
| PostgreSQL    | Base de datos                   |
| Tokio         | Runtime asíncrono               |
| JWT           | Autenticación                   |
| Argon2        | Hashing de contraseñas          |
| Serde         | Serialización y deserialización |
| Reqwest       | Cliente HTTP                    |
| Resend        | Envío de emails                 |
| Supabase      | Backend y servicio de PostgreSQL |

## 📁 Estructura del proyecto

```text
Pedro-FitBalance/
│
├── API_Calculadora_Calorias/
│   ├── src/
│   │   ├── alimentos/
│   │   │   ├── conexion.rs
│   │   │   └── mod.rs
│   │   ├── middleware/
│   │   │   ├── auth.rs
│   │   │   ├── email.rs
│   │   │   ├── mod.rs
│   │   │   └── service.rs
│   │   ├── controllers.rs
│   │   ├── db.rs
│   │   ├── repository.rs
│   │   ├── routes.rs
│   │   └── main.rs
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
│   │   ├── semanales.rs
│   │   ├── theme.rs
│   │   └── main.rs
│   ├── Cargo.toml
│   └── Cargo.lock
│
└── .gitignore
```

## ⚙️ Requisitos

Para ejecutar FitBalance necesitas:

* [Rust](https://www.rust-lang.org/tools/install)
* PostgreSQL o un servicio compatible, como [Supabase](https://supabase.com/)
* Una cuenta de [Resend](https://resend.com/) para el envío de correos electrónicos.

## 🔧 Configuración

### 1. Clonar el repositorio

```bash
git clone https://github.com/Pedro2004-svg/Pedro-FitBalance.git
cd Pedro-FitBalance
```

### 2. Configurar las variables de entorno

Dentro de `API_Calculadora_Calorias`, crea un archivo `.env` basándote en `.env.example`:

```env
DATABASE_URL=
JWT_SECRET=
RESEND_API_KEY=
```

Completa las variables con tus propios valores.

> **Importante:** el archivo `.env` contiene información sensible y no debe subirse al repositorio.

### 3. Ejecutar la API

Abre una terminal y ejecuta:

```bash
cd API_Calculadora_Calorias
cargo run
```

### 4. Ejecutar la aplicación de escritorio

En otra terminal:

```bash
cd Calculadora_Calorias
cargo run
```

La aplicación de escritorio se comunicará con la API REST para realizar las operaciones necesarias.

## 🔐 Seguridad

FitBalance utiliza diferentes mecanismos para proteger los datos y las cuentas de los usuarios:

* Las contraseñas se almacenan mediante **Argon2**.
* La autenticación de las peticiones se realiza mediante **JWT**.
* Las credenciales y secretos se gestionan mediante **variables de entorno**.
* Los archivos `.env` están excluidos del repositorio mediante `.gitignore`.

## 📌 Estado del proyecto

**Prácticamente completado.**

La funcionalidad principal de FitBalance está implementada. Actualmente, el proyecto se encuentra en una fase de pequeños ajustes y mejoras de la interfaz de usuario.

## 👨‍💻 Autor

**Pedro**

Proyecto desarrollado con Rust como aplicación de escritorio y API REST.
