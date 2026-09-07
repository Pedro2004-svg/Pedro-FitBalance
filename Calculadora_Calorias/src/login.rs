use crate::{Pantalla, egui};
use egui::{Ui, Stroke, CornerRadius, Vec2};
use serde::{Deserialize, Serialize};
use std::f32::{INFINITY};

#[derive(Serialize)]
pub struct LoginRequest {
    pub usuario: String,
    pub contrasena: String,
}

#[derive(Serialize)]
pub struct UptCorreo {
    pub correo: String,
    pub contrasena: String,
}

#[derive(Deserialize)]
struct LoginResponse {
    mensaje: String,
    success: bool,
    cuenta: String,
    correo: String,
    token: Option<String>
}

#[derive(Serialize)]
pub struct VerifyRequest {
    pub usuario: String,
    pub email: String,
    pub codigo: i64,
}

#[derive(Deserialize)]
pub struct VerifyResponse {
    pub verificado: bool,
    pub token: Option<String>,
}

pub fn show(
    ui: &mut Ui,
    usuario: &mut String,
    correo_cuenta: &mut String,
    contrasena: &mut String,
    pantalla: &mut Pantalla,
    runtime: &tokio::runtime::Runtime,
    mensaje_completo: &mut Option<(String, bool, String, String)>,
    hay_cuenta: &mut String,
    tiempo_exito: &mut Option<std::time::Instant>,
    jwt_token: &mut Option<String>,
    codigo: &mut String
) -> bool {
    
    ui.vertical_centered(|ui| {
        ui.horizontal(|ui| {
            ui.add_space(15.0);
            ui.heading("Inicio de sesion");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("Volver al Inicio").clicked() {
                    *pantalla = Pantalla::Index;
                }
            });
        });
        ui.add_space(15.0);
        ui.label("Nombre de Usuario");
        ui.add_space(4.0);
        ui.add(
            egui::TextEdit::singleline(usuario)
                .hint_text("Nombre de Usuario")
                .desired_width(280.0)
                .margin(egui::Margin::same(8)),
        );
        ui.add_space(15.0);
        ui.label("Contraseña");
        ui.add_space(4.0);
        ui.add(
            egui::TextEdit::singleline(contrasena)
                .hint_text("Contraseña")
                .desired_width(280.0)
                .margin(egui::Margin::same(8))
                .password(true),
        );
        ui.add_space(15.0);
        if ui.link("¿No tienes Cuenta? Regístrate").clicked() {
            *pantalla = Pantalla::Register;
        };
        if ui.button("Iniciar Sesion").clicked() {
            let resultado = runtime.block_on(login_api(
                usuario.clone(),
                contrasena.clone()
            ));
            match resultado {
                Ok(datos) => {
                    *mensaje_completo = Some((datos.mensaje, datos.success, datos.cuenta, datos.correo));
                }
                Err(err) => {
                    println!("{}", err);
                    *mensaje_completo = Some((
                        "Error en el servidor, inténtalo más tarde".to_string(),
                        false,
                        "".to_string(),
                        "".to_string(),
                    ));
                }
            }
        }

        // Siempre mostrar el mensaje si existe
        if let Some((mensaje, success, cuenta, correo)) = mensaje_completo {
                ui.label(
                    egui::RichText::new(mensaje.clone())
                        .color(if *success {
                            egui::Color32::GREEN
                        } else {
                            egui::Color32::RED
                        })
                        .strong(),
                );
                
                if *success && tiempo_exito.is_none() {
                    egui::Window::new("Autentificacion 2FA")
                    .collapsible(false)
                    .resizable(false)
                    .anchor(egui::Align2::CENTER_CENTER, [0.0,0.0])
                    .fixed_size([450.0, 600.0])
                    .min_size([450.0, 600.0])
                    .order(egui::Order::Foreground)
                    .frame(egui::Frame::window(&ui.global_style())
                        .inner_margin(egui::Margin::same(24)))
                    .show(ui, |ui|{
                        ui.add_space(4.0);
                        ui.vertical(|ui|{
                            ui.label("Introduce el codigo del correo");
                            ui.add(
                            egui::TextEdit::singleline(codigo)
                                .hint_text("Codigo")
                                .desired_width(INFINITY)
                                .margin(egui::Margin::same(8)),
                            );
                            let btn = egui::Button::new("Verificar codigo").stroke(Stroke::NONE)
                            .corner_radius(CornerRadius::same(8))
                            .min_size(Vec2::new(32.0, 38.0));
                            if ui.add(btn).clicked(){
                                let verificado = runtime.block_on(verify_cod(
                                    usuario.clone(),
                                    correo.clone(),
                                    codigo.parse::<i64>().unwrap().clone()
                                ));
                                match verificado{
                                    Ok(datos) => {
                                        if datos.verificado{
                                            *jwt_token = datos.token;
                                        }
                                    }

                                    Err(e) => {
                                        println!("{}", e);
                                    }
                                }
                            };
                        });
                    });
                    if !jwt_token.is_none(){
                        *hay_cuenta = cuenta.clone();
                        *correo_cuenta = correo.clone();
                        *tiempo_exito = Some(std::time::Instant::now());
                        codigo.clear();
                    }
            }
        }

        if let Some(t) = tiempo_exito {
            if t.elapsed() >= std::time::Duration::from_secs(2) {
                *tiempo_exito = None;
                *pantalla = Pantalla::Index;
            } else {
                ui.ctx().request_repaint();
            }
        }
    });
    return true;
}

async fn login_api(
    usuario: String,
    contrasena: String
) -> Result<LoginResponse, reqwest::Error> {
    let client = reqwest::Client::new();
    let respuesta = client
        .post("http://127.0.0.1:30000/login")
        .json(&LoginRequest {
            usuario,
            contrasena
        })
        .send()
        .await?;

    if !respuesta.status().is_success() {
        let status = respuesta.status();
        let body = respuesta.text().await?;

        panic!("Error backend: {} - {}", status, body);
    }
    let datos: LoginResponse = respuesta.json().await?;
    Ok(datos)
}

async fn verify_cod(
    usuario: String,
    email: String,
    codigo: i64
) -> Result<VerifyResponse, reqwest::Error> {
    let client = reqwest::Client::new();
    let respuesta = client
        .post("http://127.0.0.1:30000/verify")
        .json(&VerifyRequest {
            usuario,
            email,
            codigo
        })
        .send()
        .await?;

    if !respuesta.status().is_success() {
        let status = respuesta.status();
        let body = respuesta.text().await?;

        panic!("Error backend: {} - {}", status, body);
    }
    let datos: VerifyResponse = respuesta.json().await?;
    Ok(datos)
}