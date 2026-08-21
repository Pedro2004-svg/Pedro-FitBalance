use crate::controllers::{CrearUsuarioResult, LoginRequest, RegisterRequest};
use crate::repository::{create_user, start_sesion, User};
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use reqwest::StatusCode;
use sqlx::PgPool;

pub async fn register_user(
    pool: &PgPool,
    mut payload: RegisterRequest,
) -> Result<CrearUsuarioResult, StatusCode> {
    if payload.contrasena != payload.contrasena_confirm {
        return Ok(CrearUsuarioResult::ContraseñaNoCoincide);
    } else {
        payload.contrasena = hashear_password(payload.contrasena).map_err(|e| {
            println!("Error en el Hash: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
        create_user(&pool, &payload).await.map_err(|err| {
            println!("ERROR SQLX: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })
    }     
}

pub async fn login_user(
    pool: &PgPool,
    payload: &mut LoginRequest,
) -> Result<(bool, String), StatusCode> {
    let contrasena_usuario: Result<User, StatusCode> =
        start_sesion(&pool, &payload).await.map_err(|err| {
            println!("ERROR SQLX2: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        });
    
    if contrasena_usuario.as_ref().map(|u| u.contraseña.as_deref().unwrap_or("").to_string()).unwrap_or_default().is_empty(){
        return Ok((false, "".to_string()));
    }

    let correo = contrasena_usuario
        .as_ref()
        .map(|u| u.correo.as_deref().unwrap_or("").to_string())
        .unwrap_or_default();

    let autenticado: Result<bool, StatusCode> = verificar_password(payload.contrasena.clone(), contrasena_usuario).map_err(|e| {
        println!("ERROR SQLX3: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    });

    Ok((autenticado?, correo))
}

pub fn hashear_password(password: String) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    Ok(argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string())
}

fn verificar_password(
    password: String,
    hash_guardado: Result<User, StatusCode>,
) -> Result<bool, argon2::password_hash::Error> {
    let user = match hash_guardado {
        Ok(h) => h,
        Err(_) => return Ok(false),
    };
    let parsed_hash = PasswordHash::new(user.contraseña.as_deref().unwrap_or(""))?;
    let argon2 = Argon2::default();
    Ok(argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}
