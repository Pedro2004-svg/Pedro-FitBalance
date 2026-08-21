use crate::controllers::{alimento_register, delete_food, get_alimentos, login, obtener_tmb, register, tmb, upt_correo, upt_food, upt_usuario, upt_passw, del_usuario, verify_handler};
use crate::middleware::auth::jwt_middleware;
use crate::AppState;
use axum::{Router, routing::post, middleware, routing::delete, routing::get, routing::put};
use std::sync::Arc;
use sqlx::PgPool;

pub fn routes(pool: PgPool, jwt_secret: Arc<String>) -> Router {
    let state = AppState { pool, jwt_secret };
    
    let public: Router<AppState> = Router::new()
        .route("/login", post(login))
        .route("/verify", post(verify_handler))
        .route("/register", post(register));
        

    let protected: Router<AppState> = Router::new()
        .route("/tmb_register", post(tmb))
        .route("/get-tmb", get(obtener_tmb))
        .route("/alimento_register", post(alimento_register))
        .route("/get-alimento", get(get_alimentos))
        .route("/delete-alimento",delete(delete_food))
        .route("/actualizar-alimento",put(upt_food))
        .route("/cambio-usuario",put(upt_usuario))
        .route("/cambio-correo", put(upt_correo))
        .route("/cambio-contrasena", put(upt_passw))
        .route("/delete-usuario",delete(del_usuario))
        .layer(middleware::from_fn_with_state(state.clone(), jwt_middleware));
        

    public.merge(protected).with_state(state)
}