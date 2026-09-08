use crate::middleware::service::{login_user, register_user};
use crate::repository::{del_user, delete_alimento, get_alimento, get_tmb, get_usuario_por_correo, register_alimento, tmb_register, upt_alim, upt_email, upt_pass, upt_user,guardar_codigo_verificacion, verificar_codigo};
use axum::{Json, extract::State};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use crate::AppState;
use crate::middleware::auth::{create_token, Claims};
use chrono::{DateTime, Local};
use crate::middleware::service::hashear_password;
use crate::middleware::email::{send_confirmation_email, send_user_changes_email, send_email_changes_email, send_pass_changes_email};
use rand::RngExt;


pub enum CrearUsuarioResult {
    Creado,
    UsuarioYaExiste,
    CorreoYaExiste,
    ContraseñaNoCoincide,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub usuario: String,
    pub contrasena: String
}

#[derive(Deserialize)]
pub struct UptCorreo {
    pub correo: String,
    pub contrasena: String
}

#[derive(Deserialize)]
pub struct UptPass {
    pub contrasena_new: String,
    pub contrasena_ant: String
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub usuario: String,
    pub contrasena: String,
    pub correo: String,
    pub contrasena_confirm: String
}

#[derive(Deserialize)]
pub struct Alimento {
    pub nombre:   String,
    pub calorias: i64,
    pub usuario: String,
    pub fecha: String
}

#[derive(Serialize,sqlx::FromRow, Debug)]
pub struct AlimentoBBDD {
    pub nombre:   String,
    pub calorias: i64,
    pub id: i64, 
    pub created_at: DateTime<Local>
}

#[derive(Deserialize)]
pub struct Alimentoupt {
    pub nombre:   String,
    pub calorias: i64,
    pub id: i64
}

#[derive(Serialize)]
pub struct AlimentoResponse {
    pub mensaje_alimento: String
}

#[derive(Serialize)]
pub struct LoginResponse {
    mensaje: String,
    success: bool,
    cuenta: String,
    correo: String,
    token: Option<String>,
    requiere_2fa: bool
}

#[derive(Deserialize)]
pub struct TMBRequest {
    pub calorias: String,
    pub usuario: String,
}

#[derive(Deserialize)]
pub struct VerifyRequest {
    pub email: String,
    pub codigo: i64,
}

#[derive(Serialize)]
pub struct VerifyResponse {
    pub verificado: bool,
    pub token: Option<String>,
}

#[derive(Serialize)]
pub struct TMBResponse {
    pub tmb: String,
    pub nombre_usuario: String,
}

pub async fn login(
    State(state): State<AppState>,
    Json(mut payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    if payload.usuario.is_empty() || payload.contrasena.is_empty() {
        return Ok(Json(LoginResponse {
            mensaje: "Introduzca todos los campos".to_string(),
            success: false,
            cuenta: "".to_string(),
            correo: "".to_string(),
            token: None,
            requiere_2fa: false
        }));
    }

    let (sesion_activa, correo_cuenta) = login_user(&state.pool, &mut payload).await?;


    if sesion_activa {
        let codigo: i32 = rand::rng().random_range(100000..999999);

        if let Err(e) = guardar_codigo_verificacion(&state.pool, &correo_cuenta, codigo).await {
            eprintln!("Error guardando código de verificación: {e}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }

        tokio::spawn({
            let email = correo_cuenta.clone();
            async move {
                if let Err(e) = send_confirmation_email(&email, codigo).await {
                    eprintln!("Error enviando email de confirmación: {e}"); // sigue funcionando igual
                }
            }
        });
        Ok(Json(LoginResponse {
            mensaje: "Código de verificación enviado a tu email".to_string(),
            success: true,
            cuenta: payload.usuario.clone(),
            correo: correo_cuenta.clone(),
            token: None,
            requiere_2fa: true,
        }))
    } else {
        Ok(Json(LoginResponse {
            mensaje: "Usuario o contraseña incorrecto".to_string(),
            success: false,
            cuenta: "".to_string(),
            correo: "".to_string(),
            token: None,
            requiere_2fa: false
        }))
    }
}

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    if payload.usuario.is_empty() || payload.contrasena.is_empty() || payload.contrasena_confirm.is_empty() ||payload.correo.is_empty() {
        Ok(Json(LoginResponse {
            mensaje: "Por favor introduzca todos los campos para registrarse".to_string(),
            success: false,
            cuenta: "".to_string(),
            correo: "".to_string(),
            token: None,
            requiere_2fa: false
        }))
    } else {
        let cuenta = payload.usuario.clone();
        let token = create_token(&payload.usuario, &state.jwt_secret)?;
        let registro = register_user(&state.pool, payload).await?;
        match registro {
            CrearUsuarioResult::Creado => Ok(Json(LoginResponse {
                mensaje: "Usuario registrado correctamente".to_string(),
                success: true,
                cuenta: cuenta,
                correo: "".to_string(),
                token: Some(token),
                requiere_2fa: false
            })),
            CrearUsuarioResult::UsuarioYaExiste => Ok(Json(LoginResponse {
                mensaje: "El usuario ya existe".to_string(),
                success: false,
                cuenta: "".to_string(),
                correo: "".to_string(),
                token: None,
                requiere_2fa: false
            })),
            CrearUsuarioResult::CorreoYaExiste => Ok(Json(LoginResponse {
                mensaje: "El correo ya esta asignado a otro usuario".to_string(),
                success: false,
                cuenta: "".to_string(),
                correo: "".to_string(),
                token: None,
                requiere_2fa: false
            })),
            CrearUsuarioResult::ContraseñaNoCoincide => Ok(Json(LoginResponse {
                mensaje: "Las contraseñas no coinciden".to_string(),
                success: false,
                cuenta: "".to_string(),
                correo: "".to_string(),
                token: None,
                requiere_2fa: false
            })),
        }
    }
}

pub async fn tmb(
    State(state): State<AppState>,
    claims: Claims,
    Json(mut payload): Json<TMBRequest>,
) -> Result<Json<TMBResponse>, StatusCode> {
    // Ignoramos el usuario que venga en el body: el dueño del recurso es
    // siempre el usuario autenticado por el JWT, nunca un dato del cliente.
    payload.usuario = claims.sub;
    let tmb_registrado: Result<bool, StatusCode> =
        tmb_register(&state.pool, &payload).await.map_err(|e| {
            eprintln!("ERROR SQLX: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        });
    if tmb_registrado? {
        Ok(Json(TMBResponse {
            tmb: payload.calorias,
            nombre_usuario: payload.usuario,
        }))
    } else {
        Ok(Json(TMBResponse {
            tmb: "".to_string(),
            nombre_usuario: payload.usuario,
        }))
    }
}

pub async fn alimento_register(
    State(state): State<AppState>,
    claims: Claims,
    Json(mut payload): Json<Alimento>
) -> Result<Json<AlimentoResponse>, StatusCode> {
    payload.usuario = claims.sub;
    let registrado = register_alimento(&state.pool, &payload).await.map_err(|e| {
        eprintln!("ERROR SQLX: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    if registrado {
        Ok(Json(AlimentoResponse { mensaje_alimento: "Alimento registrado correctamente".to_string() }))
    } else {
        Err(StatusCode::BAD_REQUEST)
    }
}

pub async fn get_alimentos(
    State(state): State<AppState>,
    claims: Claims
) -> Result<Json<Vec<AlimentoBBDD>>, StatusCode>{
    let alimento: Vec<AlimentoBBDD> = get_alimento(&state.pool, claims.sub).await.map_err(|e|{
        eprintln!("ERROR SQLX: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(alimento))
}

pub async fn delete_food(
    State(state): State<AppState>,
    claims: Claims,
    payload: String
) -> Result<String, StatusCode>{
    let id_alimento: i64 = payload.parse().map_err(|_| StatusCode::BAD_REQUEST)?;

    let borrado = delete_alimento(&state.pool, id_alimento, claims.sub).await.map_err(|e|{
        eprintln!("ERROR SQLX: {:?}",e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    if borrado {
        Ok("Alimento eliminado correctamente".to_string())
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn obtener_tmb(
    State(state): State<AppState>,
    claims: Claims
) -> Result<Json<String>, StatusCode>{
    let get_tmb: Result<String, StatusCode> = get_tmb(&state.pool, claims.sub).await.map_err(|e|{
        eprintln!("ERROR SQLX: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    });

    match get_tmb{
        Ok(valor) => {
            Ok(Json(valor))
        }
        Err(e) =>{
            Ok(Json(format!("Error en el Servidor, {}", e)))
        }
    }
}

pub async fn upt_food(
    State(state): State<AppState>,
    claims: Claims,
    Json(payload): Json<Alimentoupt>
)-> Result<String, StatusCode>{
    let alimentos:AlimentoBBDD = AlimentoBBDD{
        nombre: payload.nombre,
        calorias: payload.calorias,
        id: payload.id,
        created_at: Local::now(),
    };
    
    upt_alim(&state.pool, &alimentos, claims.sub).await.map_err(|e|{
        eprintln!("ERROR SQLX: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok("Alimento actualizado correctamente".to_string())
}

pub async fn upt_usuario(
    State(state): State<AppState>,
    claims: Claims,
    Json(payload): Json<LoginRequest>
)->Result<Json<(String, Option<String>)>, StatusCode>{

    //Verifica que la contraseña introducida sea la correcta
    let (es_correcta, correo) = login_user(&state.pool, &mut LoginRequest { usuario: claims.sub.clone(), contrasena: payload.contrasena.clone() }).await?;

    if !es_correcta{
        return Ok(Json(("Contrasñea incorrecta".to_string(), None)))
    }

    let upt_user: Result<String, StatusCode> = upt_user(&state.pool, &payload.usuario, claims.sub).await.map_err(|e|{
        eprintln!("Error SQLX: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    });

    let token = create_token(&payload.usuario, &state.jwt_secret)?;
    
    tokio::spawn({
        let email = correo.clone();
        async move {
            if let Err(e) = send_user_changes_email(&email).await {
                eprintln!("Error enviando email de confirmación: {e}");
            }
        }
    });

    match upt_user{
        Ok(mensaje) =>{
            return Ok(Json((mensaje, Some(token))))
        }

        Err(e) =>{
            return Ok(Json((e.to_string(), None)))
        }
    }

    
}

pub async fn upt_correo(
    State(state): State<AppState>,
    claims: Claims,
    Json(payload): Json<UptCorreo>
)->Result<String, StatusCode>{

    //Verifica que la contraseña introducida sea la correcta
    let (es_correcta, correo) = login_user(&state.pool, &mut LoginRequest { usuario: claims.sub.clone(), contrasena: payload.contrasena.clone() }).await?;

    if !es_correcta{
        return Ok("Contrasñea incorrecta".to_string())
    }

    let upt_email: Result<String, StatusCode> = upt_email(&state.pool, &payload.correo, claims.sub).await.map_err(|e|{
        eprintln!("Error SQLX: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    });

    tokio::spawn({
        let email = correo.clone();
        async move {
            if let Err(e) = send_email_changes_email(&email).await {
                eprintln!("Error enviando email de confirmación: {e}");
            }
        }
    });

    match upt_email{
        Ok(mensaje) =>{
            return Ok(mensaje)
        }

        Err(e) =>{
            return Ok(e.to_string())
        }
    }
}


pub async fn upt_passw(
    State(state): State<AppState>,
    claims: Claims,
    Json(mut payload): Json<UptPass>
)->Result<String, StatusCode>{

    //Verifica que la contraseña introducida sea la correcta
    let (es_correcta, correo) = login_user(&state.pool, &mut LoginRequest { usuario: claims.sub.clone(), contrasena: payload.contrasena_ant.clone() }).await?;

    if !es_correcta{
        return Ok("Contrasñea incorrecta".to_string())
    }

    //Hashea la contraseña
    payload.contrasena_new = hashear_password(payload.contrasena_new).map_err(|e| {
            eprintln!("Error en el Hash: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let upt_pass: Result<String, StatusCode> = upt_pass(&state.pool, &payload.contrasena_new, claims.sub).await.map_err(|e|{
        eprintln!("Error SQLX: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    });

    tokio::spawn({
        let email = correo.clone();
        async move {
            if let Err(e) = send_pass_changes_email(&email).await {
                eprintln!("Error enviando email de confirmación: {e}");
            }
        }
    });

    match upt_pass{
        Ok(mensaje) =>{
            return Ok(mensaje)
        }

        Err(e) =>{
            return Ok(e.to_string())
        }
    }
}

pub async fn del_usuario(
    State(state): State<AppState>,
    claims: Claims,
    contrasena: String
) -> Result<Json<bool>, StatusCode>{

    let (es_correcta, _) = login_user(&state.pool, &mut LoginRequest { usuario: claims.sub.clone(), contrasena: contrasena.clone() }).await?;

    if !es_correcta{
        return Ok(Json(false))
    }

    let delete_user = del_user(&state.pool, claims.sub).await.map_err(|e|{
        eprintln!("Error SQLX: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    });

    match delete_user{
        Ok(correcto) =>{
            return Ok(Json(correcto))
        }

        Err(e) =>{
            eprintln!("{}", e.to_string());
            return Ok(Json(false))
        }
    }
}

pub async fn verify_handler(
    State(state): State<AppState>,
    Json(payload): Json<VerifyRequest>,
) -> Result<Json<VerifyResponse>, (StatusCode, String)> {
    let es_valido = verificar_codigo(&state.pool, &payload.email, payload.codigo)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if !es_valido {
        return Err((StatusCode::UNAUTHORIZED, "Código incorrecto o expirado".to_string()));
    }

    // El token se emite para el usuario dueño real del email verificado,
    // nunca para el "usuario" que venga en el body (evita suplantación).
    let usuario_real = get_usuario_por_correo(&state.pool, &payload.email)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::UNAUTHORIZED, "Cuenta no encontrada".to_string()))?;

    sqlx::query("DELETE FROM verificaciones WHERE email = $1")
        .bind(&payload.email)
        .execute(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let token = create_token(&usuario_real, &state.jwt_secret)
        .map_err(|status| (status, "Error generando el token".to_string()))?;

    Ok(Json(VerifyResponse {
        verificado: true,
        token: Some(token),
    }))
}