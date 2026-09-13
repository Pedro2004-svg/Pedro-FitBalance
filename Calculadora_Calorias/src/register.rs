use crate::{Pantalla, egui};
use egui::Ui;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct RegisterRequest {
    usuario: String,
    contrasena: String,
    correo: String,
    contrasena_confirm: String
}

#[derive(Deserialize)]
struct RegisterResponse {
    mensaje: String,
    success: bool,
    cuenta: String,
}

pub fn show(
    ui: &mut Ui,
    usuario: &mut String,
    contrasena: &mut String,
    contrasena_confirm: &mut String,
    correo: &mut String,
    pantalla: &mut Pantalla,
    runtime: &tokio::runtime::Runtime,
    mensaje_completo: &mut Option<(String, bool, String, String)>,
    tiempo_exito: &mut Option<std::time::Instant>,
) -> bool {
    ui.vertical_centered(|ui| {
        ui.horizontal(|ui| {
            ui.add_space(15.0);
            ui.heading("Registro de usuario");
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
                .hint_text("••••••••")
                .desired_width(280.0)
                .margin(egui::Margin::same(8))
                .password(true),
        );

        ui.add_space(15.0);
        ui.label("Confirmar contraseña");
        ui.add_space(4.0);
        ui.add(
            egui::TextEdit::singleline(contrasena_confirm)
                .hint_text("••••••••")
                .desired_width(280.0)
                .margin(egui::Margin::same(8))
                .password(true),
        );
        ui.add_space(15.0);
        ui.label("Correo Electronico");
        ui.add_space(4.0);
        ui.add(
            egui::TextEdit::singleline(correo)
                .hint_text("Correo Electronico")
                .desired_width(280.0)
                .margin(egui::Margin::same(8)),
        );
        ui.add_space(15.0);
        if ui.link("¿Ya tienes Cuenta? Inicia Sesion").clicked() {
            *pantalla = Pantalla::Login;
        }

        if ui.button("Registrate").clicked() {
            let resultado = runtime.block_on(create_user(
                usuario.clone(),
                contrasena.clone(),
                correo.clone(),
                contrasena_confirm.clone(),
            ));

            match resultado {
                Ok(datos) => *mensaje_completo = Some((datos.mensaje, datos.success, datos.cuenta, "".to_string())),
                Err(err) => {
                    println!("Error en el registro {}", err);
                    *mensaje_completo = Some((
                        "Error en el servidor, inténtalo más tarde".to_string(),
                        false,
                        "".to_string(),
                        "".to_string(),
                    ));
                }
            }
        }
        let _correo = "".to_string();
        if let Some((mensaje, success, _cuenta, _correo)) = mensaje_completo.clone() {
            ui.label(
                egui::RichText::new(mensaje.clone())
                    .color(if success {
                        egui::Color32::GREEN
                    } else {
                        egui::Color32::RED
                    })
                    .strong(),
            );

            if success && tiempo_exito.is_none() {
                *tiempo_exito = Some(std::time::Instant::now());
            }
        }

        if let Some(t) = tiempo_exito {
            if t.elapsed() >= std::time::Duration::from_secs(2) {
                *tiempo_exito = None;
                *pantalla = Pantalla::Login;
            } else {
                ui.ctx().request_repaint();
            }
        }
    });
    return true;
}

async fn create_user(
    usuario: String,
    contrasena: String,
    correo: String,
    contrasena_confirm: String
) -> Result<RegisterResponse, String> {
    let client = reqwest::Client::new();
    let respuesta = client
        .post("http://127.0.0.1:30000/register")
        .json(&RegisterRequest {
            usuario,
            contrasena,
            correo,
            contrasena_confirm
        })
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if !respuesta.status().is_success() {
        // Antes esto hacía panic!() y cerraba toda la app de escritorio ante
        // cualquier error del backend.
        let status = respuesta.status();
        let body = respuesta.text().await.unwrap_or_default();
        return Err(format!("Error backend: {} - {}", status, body));
    }
    
    let datos: RegisterResponse = respuesta.json().await.map_err(|e| e.to_string())?;
    Ok(datos)
}