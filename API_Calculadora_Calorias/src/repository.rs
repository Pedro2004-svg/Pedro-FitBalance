use crate::controllers::{Alimento, AlimentoBBDD, CrearUsuarioResult, LoginRequest, RegisterRequest, TMBRequest};
use sqlx::{PgPool};
use chrono::{NaiveDate, DateTime, Utc, Duration};

#[derive(Default)]
pub struct User {
    pub contraseña: Option<String>,
    pub correo: Option<String>
}
pub async fn create_user(
    pool: &PgPool,
    payload: &RegisterRequest,
) -> Result<CrearUsuarioResult, sqlx::Error> {
    let usuario_existente: Option<sqlx::postgres::PgRow> = sqlx::query(
        r#"
        SELECT nombre_usuario FROM usuarios_registrados WHERE nombre_usuario = ($1)
        "#,
    )
    .bind(&payload.usuario)
    .fetch_optional(pool)
    .await?;

    if usuario_existente.is_some() {
        println!("Entro usuario");
        return Ok(CrearUsuarioResult::UsuarioYaExiste);
    }

    println!("Llego a esta parte");
    let correo_existente: Option<sqlx::postgres::PgRow> = sqlx::query(
        r#"
        SELECT correo FROM usuarios_registrados WHERE correo = ($1)
        "#,
    )
    .bind(&payload.correo)
    .fetch_optional(pool)
    .await?;
    if correo_existente.is_some() {
        println!("Entro correo");
        return Ok(CrearUsuarioResult::CorreoYaExiste);
    }

    println!("Llego a la 2º parte");
    sqlx::query(
        r#"
        INSERT INTO usuarios_registrados (nombre_usuario, contraseña, correo)
        Values ($1, $2, $3)
        "#,
    )
    .bind(&payload.usuario)
    .bind(&payload.contrasena)
    .bind(&payload.correo)
    .execute(pool)
    .await?;

    Ok(CrearUsuarioResult::Creado)
}

pub async fn start_sesion(
    pool: &PgPool,
    payload: &LoginRequest,
) -> Result<User, sqlx::Error> {

    let datos: Option<User> = sqlx::query_as!(
        User,
        "SELECT contraseña, correo FROM usuarios_registrados WHERE nombre_usuario = ($1) ",
        &payload.usuario
    )
    .fetch_optional(pool)
    .await?;

    match datos {
        Some(user) => Ok(User{
            contraseña: user.contraseña, 
            correo: user.correo}),
            
        None => Ok(User { contraseña: None, correo: None}),
    }
}

pub async fn tmb_register(pool: &PgPool, payload: &TMBRequest) -> Result<bool, sqlx::Error> {
    println!("Entro al registro del TMB");
    let registro = sqlx::query(
        r#"
                    UPDATE usuarios_registrados SET tmb = ($1) WHERE nombre_usuario = ($2)
                "#,
    )
    .bind(&payload.calorias)
    .bind(&payload.usuario)
    .fetch_optional(pool)
    .await?;

    match registro {
        Some(_registro) => Ok(true),
        None => Ok(false),
    }
}

pub async fn register_food(
    pool: &PgPool,
    description: String,
    calorias: f64,
) -> Result<(), sqlx::Error> {
    let buscador = sqlx::query(
        r#"
        SELECT descripcion FROM alimentos WHERE descripcion = $1
        "#,
    )
    .bind(&description)
    .fetch_all(pool)
    .await?;

    if !buscador.is_empty() {
        println!("Se ha encontrado");
        return Ok(());
    }

    let _registro = sqlx::query(
        r#"
        INSERT INTO alimentos (descripcion, calorias)
        Values ($1, $2)
        "#,
    )
    .bind(&description)
    .bind(&calorias)
    .execute(pool)
    .await;

    Ok(())
}

pub async fn register_alimento(
    pool: &PgPool,
    payload: &Alimento
)-> Result<bool, sqlx::Error>{
    println!("entro {},{}", payload.calorias, payload.nombre);
    let fecha = NaiveDate::parse_from_str(
        &payload.fecha[..10],
        "%Y-%m-%d"
    ).unwrap();
    let _registro = sqlx::query(
        r#"
        INSERT INTO alimentos_usuario (created_at,id_usuario, calorias, nombre)
        Values ($1 ,(SELECT id FROM usuarios_registrados WHERE nombre_usuario = $2), $3, $4)
        "#,
    )
    .bind(fecha)
    .bind(&payload.usuario)
    .bind(&payload.calorias)
    .bind(&payload.nombre)
    .execute(pool)
    .await;

    Ok(true)
}

pub async fn get_alimento(
    pool: &PgPool,
    payload: String
) -> Result<Vec<AlimentoBBDD>, sqlx::Error> {

    let alimentos = sqlx::query_as::<_, AlimentoBBDD>(
    r#"
    SELECT id, calorias, nombre, created_at
    FROM alimentos_usuario
    WHERE id_usuario = (
        SELECT id
        FROM usuarios_registrados
        WHERE nombre_usuario = $1
    ) 
    "#
)
.bind(payload)
.fetch_all(pool)
.await?;

Ok(alimentos)
}

pub async fn delete_alimento(
    pool: &PgPool,
    payload: i64,
) -> Result<bool, sqlx::Error>{
    println!("Llego al repo en el delete");
    let _delete = sqlx::query(
    r#"
            DELETE FROM alimentos_usuario WHERE id = $1
        "#
    )
    .bind(payload)
    .execute(pool)
    .await;

    Ok(true)
}

pub async fn get_tmb(
    pool: &PgPool,
    payload: String,
) -> Result<String, sqlx::Error> {
    let tmb: Option<String> = sqlx::query_scalar!(
        "
        SELECT tmb
        FROM usuarios_registrados
        WHERE nombre_usuario = $1
        ",
        payload
    )
    .fetch_one(pool)
    .await?;

    match tmb {
        Some(valor) => {
            Ok(valor)
        }
        None => {
            Ok("".to_string())
        }
    }
}

pub async fn upt_alim(
    pool: &PgPool,
    payload: &AlimentoBBDD,
) -> Result<bool, sqlx::Error> {

    if !payload.nombre.is_empty(){
        sqlx::query(
        r#"
                UPDATE alimentos_usuario SET nombre = ($1) WHERE id = ($2)
            "#,
        )
        .bind(&payload.nombre)
        .bind(&payload.id)
        .fetch_optional(pool)
        .await?;
    } 
    
    if !payload.calorias.to_string().is_empty(){
        let calorias = payload.calorias;
        sqlx::query(
        r#"
                UPDATE alimentos_usuario SET calorias = ($1) WHERE id = ($2)
            "#,
        )
        .bind(&calorias)
        .bind(&payload.id)
        .fetch_optional(pool)
        .await?;
    }

    Ok(true)
}

pub async fn upt_user(
    pool: &PgPool,
    usuario_nuevo: &String,
    usuario_ant: String
) -> Result<String, sqlx::Error>{
    let usuario_existente: Option<sqlx::postgres::PgRow> = sqlx::query(
        r#"
        SELECT nombre_usuario FROM usuarios_registrados WHERE nombre_usuario = ($1)
        "#,
    )
    .bind(&usuario_nuevo)
    .fetch_optional(pool)
    .await?;

    if usuario_existente.is_some() {
        return Ok("Este usuario ya esxite".to_string());
    }

    if !usuario_nuevo.is_empty(){
        sqlx::query(
            r#"
                UPDATE usuarios_registrados SET nombre_usuario = ($1) WHERE nombre_usuario = ($2)
            "#,
        )
        .bind(&usuario_nuevo)
        .bind(usuario_ant)
        .fetch_optional(pool)
        .await?;
    }
    
    Ok("Usuario actualizado correctamente".to_string())
}

pub async fn upt_email(
    pool: &PgPool,
    correo: &String,
    usuario: String
) -> Result<String, sqlx::Error>{
    let correo_existente: Option<sqlx::postgres::PgRow> = sqlx::query(
        r#"
        SELECT correo FROM usuarios_registrados WHERE correo = ($1)
        "#,
    )
    .bind(&correo)
    .fetch_optional(pool)
    .await?;

    if correo_existente.is_some() {
        return Ok("Este correo ya esxite".to_string());
    }

    if !correo.is_empty(){
        sqlx::query(
            r#"
                UPDATE usuarios_registrados SET correo = ($1) WHERE nombre_usuario = ($2)
            "#,
        )
        .bind(&correo)
        .bind(usuario)
        .fetch_optional(pool)
        .await?;
    }
    
    Ok("Correo actualizado correctamente".to_string())
}

pub async fn upt_pass(
    pool: &PgPool,
    contrasena: &String,
    usuario: String
) -> Result<String, sqlx::Error>{
    if !contrasena.is_empty(){
        println!("Entre en el Update, {}, con el usuario, {}", contrasena, usuario);
        sqlx::query(
            r#"
                UPDATE usuarios_registrados SET contraseña = ($1) WHERE nombre_usuario = ($2)
            "#,
        )
        .bind(&contrasena)
        .bind(usuario)
        .fetch_optional(pool)
        .await?;
    }
    
    Ok("Contraseña actualizada correctamente".to_string())
}

pub async fn del_user(
    pool: &PgPool,
    usuario: String
) -> Result<bool, sqlx::Error>{
    if !usuario.is_empty(){
        sqlx::query(
            r#"
                DELETE FROM alimentos_usuario WHERE id_usuario = (SELECT id FROM usuarios_registrados WHERE nombre_usuario = ($1))
            "#,
        )
        .bind(usuario.clone())
        .fetch_optional(pool)
        .await?;

        sqlx::query(
            r#"
                DELETE FROM usuarios_registrados WHERE id = (SELECT id FROM usuarios_registrados WHERE nombre_usuario = ($1))
            "#,
        )
        .bind(usuario.clone())
        .fetch_optional(pool)
        .await?;
    }
    Ok(true)
}

pub async fn guardar_codigo_verificacion(
    pool: &PgPool,
    email: &str,
    codigo: i32,
) -> Result<(), sqlx::Error> {

    let expira_en: DateTime<Utc> = Utc::now() + Duration::minutes(10); // expira en 10 minutos

    sqlx::query(
        r#"
        INSERT INTO verificaciones (email, codigo, expira_en)
        VALUES ($1, $2, $3)
        ON CONFLICT(email) DO UPDATE SET
            codigo = excluded.codigo,
            expira_en = excluded.expira_en
        "#,
    )
    .bind(email)
    .bind(codigo)
    .bind(expira_en)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn verificar_codigo(
    pool: &PgPool,
    email: &str,
    codigo_recibido: i64,
) -> Result<bool, sqlx::Error> {
    let row: Option<(i64, chrono::DateTime<Utc>)> = sqlx::query_as(
        "SELECT codigo, expira_en FROM verificaciones WHERE email = $1",
    )
    .bind(email)
    .fetch_optional(pool)
    .await?;

    Ok(match row {
        Some((codigo, expira_en)) => codigo == codigo_recibido && Utc::now() < expira_en,
        None => false,
    })
}