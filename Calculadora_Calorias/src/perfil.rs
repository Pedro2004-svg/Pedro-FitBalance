use std::f32::INFINITY;

use crate::{egui, Pantalla};
use crate::login::{LoginRequest, UptCorreo};
use egui::{Ui, RichText, Color32, Stroke, CornerRadius, Vec2};
use crate::resumen::card;
use crate::ModalTipo;

use serde::{ Serialize};

#[derive(Serialize)]
struct UptPass {
    contrasena_new: String,
    contrasena_ant: String
}

pub fn show(ui: &mut Ui, hay_cuenta: &mut String, modal: &mut ModalTipo, usuario_mod: &mut String, correo: &mut String, contrasena_confirm: &mut String, 
    contrasena :&mut String, jwt_token: &mut Option<String>,runtime: &tokio::runtime::Runtime, mensaje: &mut String,  pantalla: &mut Pantalla, tiempo_exito: &mut Option<std::time::Instant>){
    let hay_modal_abierta = *modal != ModalTipo::Ninguna;
    ui.add_enabled_ui(!hay_modal_abierta, |ui|{
        egui::ScrollArea::vertical().show(ui, |ui|{
            ui.vertical_centered(|ui|{
                ui.label(RichText::new(format!("Bienvenido, {}", hay_cuenta)).size(20.0).color(Color32::from_rgb(220, 80, 80)).strong());
                ui.label(RichText::new("Configuración de tu cuenta").size(13.0).color(Color32::from_rgb(220, 80, 80)));
                ui.add_space(24.0);
                ui.separator();
                ui.add_space(24.0);
                card(ui, |ui|{
                    ui.set_max_width(640.0);
                    ui.label(RichText::new("Cuenta").size(15.0).strong());
                    ui.add_space(15.0);
                    ui.separator();
                    ui.add_space(15.0);

                    ui.allocate_ui_with_layout(egui::Vec2::new(ui.available_width(), 38.0), egui::Layout::left_to_right(egui::Align::Center),|ui| {
                        ui.label("Nombre de usuario");
                        ui.label(&*hay_cuenta);
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui|{
                            let btn = egui::Button::new("Cambiar Usuario").stroke(Stroke::NONE)
                            .corner_radius(CornerRadius::same(8))
                            .min_size(Vec2::new(32.0, 38.0));
                            if ui.add(btn).clicked(){
                                *modal = ModalTipo::Usuario;
                                *usuario_mod = "".to_string();
                                *contrasena = "".to_string();
                                *contrasena_confirm = "".to_string();
                                *mensaje = "".to_string();
                            };
                        })
                    });
                    ui.add_space(15.0);
                    ui.separator();
                    ui.add_space(15.0);
                    
                    ui.allocate_ui_with_layout(egui::Vec2::new(ui.available_width(), 38.0), egui::Layout::left_to_right(egui::Align::Center),|ui| {
                        ui.label("Correo");
                        ui.label(&*correo);
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui|{
                            let btn = egui::Button::new("Cambiar Correo").stroke(Stroke::NONE)
                            .corner_radius(CornerRadius::same(8))
                            .min_size(Vec2::new(32.0, 38.0));
                            if ui.add(btn).clicked(){
                                *modal = ModalTipo::Correo;
                                *usuario_mod = "".to_string();
                                *contrasena = "".to_string();
                                *contrasena_confirm = "".to_string();
                                *mensaje = "".to_string();
                            };
                        })
                    });

                    ui.add_space(15.0);
                    ui.separator();
                    ui.add_space(15.0);
                    
                    ui.allocate_ui_with_layout(egui::Vec2::new(ui.available_width(), 38.0), egui::Layout::left_to_right(egui::Align::Center),|ui| {
                        ui.label("Contraseña");
                        ui.label("*******");
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui|{
                            let btn = egui::Button::new("Cambiar Contraseña").stroke(Stroke::NONE)
                            .corner_radius(CornerRadius::same(8))
                            .min_size(Vec2::new(32.0, 38.0));
                            if ui.add(btn).clicked(){
                                *modal = ModalTipo::Contrasena;
                                *usuario_mod = "".to_string();
                                *contrasena = "".to_string();
                                *contrasena_confirm = "".to_string();
                                *mensaje = "".to_string();
                            };
                        })
                    });
                });

                ui.add_space(24.0);
                card(ui, |ui|{
                    ui.set_max_width(640.0);
                    ui.label(RichText::new("Zona de peligro").size(15.0).strong());
                    ui.add_space(15.0);
                    ui.separator();
                    ui.add_space(15.0);

                    ui.allocate_ui_with_layout(egui::Vec2::new(ui.available_width(), 38.0), egui::Layout::left_to_right(egui::Align::Center),|ui| {
                        ui.vertical(|ui|{
                            ui.label("Eliminar Cuenta");
                            ui.label("Elimina permanentemente tu cuenta y todos tus datos");
                        });
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui|{
                            let btn = egui::Button::new("Eliminar Cuenta").stroke(Stroke::NONE)
                            .corner_radius(CornerRadius::same(8))
                            .min_size(Vec2::new(32.0, 38.0));
                            if ui.add(btn).clicked(){
                            *modal = ModalTipo::EliminarCuenta;
                            };
                        });
                    });
                });
            });
        });
    });
    mostrar_modal(ui, modal, hay_cuenta, contrasena, usuario_mod, correo, contrasena_confirm, jwt_token, runtime, mensaje, pantalla, tiempo_exito);
}

fn mostrar_modal(ui: &egui::Context, modal: &mut ModalTipo, usuario: &mut String, contrasena :&mut String, usuario_mod:&mut String, correo:&mut String, contrasena_confirm: &mut String, 
    jwt_token: &mut Option<String>, runtime: &tokio::runtime::Runtime, mensaje: &mut String, pantalla: &mut Pantalla, tiempo_exito: &mut Option<std::time::Instant>){
    if *modal == ModalTipo::Ninguna{return;}

    let titulo = match modal {
        ModalTipo::Contrasena => "Cambiar contraseña",
        ModalTipo::Correo => "Cambiar Correo",
        ModalTipo::EliminarCuenta => "Eliminar Cuenta",
        ModalTipo::Usuario => "Cambiar usuario",
        ModalTipo::Ninguna => "",
    };

    // Overlay que bloquea clicks detrás
    let overlay_response = egui::Area::new(egui::Id::new("modal_overlay"))
        .order(egui::Order::Middle)
        .fixed_pos(egui::Pos2::ZERO)
        .show(ui, |ui| {
            let screen_rect = ui.content_rect();
            ui.painter().rect_filled(
                screen_rect,
                0.0,
                egui::Color32::from_black_alpha(150),
            );
            ui.allocate_rect(screen_rect, egui::Sense::click())
        })
        .inner;

    if overlay_response.clicked(){
        *modal = ModalTipo::Ninguna;
    }

    let frame = if *modal == ModalTipo::EliminarCuenta{
        egui::Frame::window(&ui.global_style())
        .stroke(Stroke::new(1.0, Color32::from_rgb(127, 29, 29)))
        .inner_margin(egui::Margin::same(24))
    }else{
        egui::Frame::window(&ui.global_style())
        .inner_margin(egui::Margin::same(24))
    };

    egui::Window::new(titulo)
    .collapsible(false)
    .resizable(false)
    .anchor(egui::Align2::CENTER_CENTER, [0.0,0.0])
    .fixed_size([450.0, 600.0])
    .min_size([450.0, 600.0])
    .order(egui::Order::Foreground)
    .frame(frame)
    .show(ui, |ui|{
        match modal{
            ModalTipo::Correo => {
                modal_correo(ui, correo, contrasena, usuario_mod, modal, jwt_token, runtime, mensaje, pantalla, tiempo_exito);
            }

            ModalTipo::Contrasena => {
                modal_contrasena(ui, contrasena, usuario_mod, modal, contrasena_confirm, jwt_token, runtime, mensaje, pantalla, tiempo_exito);
            }

            ModalTipo::EliminarCuenta => {
                modal_eliminar(ui, usuario, contrasena, modal, jwt_token, pantalla, runtime, mensaje);
            }

            ModalTipo::Usuario => {
                modal_usuario(ui, usuario, contrasena, usuario_mod, modal, jwt_token, runtime, mensaje, pantalla, tiempo_exito);
            }

            ModalTipo::Ninguna => {}
        }
    });
}


fn avatar(ui: &mut egui::Ui, nombre: &str) {
    let tamano = 44.0;
    let (rect, _) = ui.allocate_exact_size(
        egui::Vec2::splat(tamano),
        egui::Sense::hover(),
    );

    // Círculo de fondo
    ui.painter().circle_filled(
        rect.center(),
        tamano / 2.0,
        Color32::from_rgb(20, 40, 80), // ACCENT_BG
    );

    // Iniciales (primeras 2 letras en mayúscula)
    let iniciales: String = nombre
        .chars()
        .next()
        .map(|c| c.to_uppercase().to_string())
        .unwrap_or_default();

    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        &iniciales,
        egui::FontId::proportional(16.0),
        Color32::from_rgb(59, 130, 246),
    );
}

fn modal_usuario(ui: &mut Ui, usuario: &mut String, contrasena :&mut String, usuario_mod: &mut String, modal: &mut ModalTipo, jwt_token: &mut Option<String>, 
    runtime: &tokio::runtime::Runtime, mensaje: &mut String, pantalla: &mut Pantalla, tiempo_exito: &mut Option<std::time::Instant>){
    ui.add_space(4.0);
    egui::Frame::new()
    .fill(Color32::from_rgb(15, 15, 15))
    .stroke(Stroke::new(0.2, Color32::from_rgba_premultiplied(255, 255, 255, 12)))
    .corner_radius(CornerRadius::same(8))
    .inner_margin(egui::Margin::same(12))
    .show(ui, |ui|{
        ui.set_min_width(ui.available_width());
        ui.horizontal(|ui|{
            avatar(ui, &*usuario);
            ui.vertical(|ui|{
                ui.label("Usuario actual");
                ui.label(RichText::new(format!("{}", usuario)).strong());
            })
        });
    });
    ui.add_space(15.0);
    ui.vertical(|ui|{
        ui.label("Nombre de usuario");
        ui.add(
        egui::TextEdit::singleline( usuario_mod)
            .hint_text("Nuevo Usuario")
            .desired_width(INFINITY)
            .margin(egui::Margin::same(8)),
        );
        ui.add_space(10.0);
        ui.label("Contraseña actual");
        ui.add(
        egui::TextEdit::singleline(contrasena)
            .hint_text("Contraseña Actual")
            .desired_width(INFINITY)
            .margin(egui::Margin::same(8)),
        );
        ui.add_space(4.0);
        ui.label("Necesaria para confirmar el cambio.");
        ui.add_space(15.0);
        ui.horizontal(|ui|{
            let mut btn = egui::Button::new("Cancelar").stroke(Stroke::NONE)
            .corner_radius(CornerRadius::same(8))
            .min_size(Vec2::new(ui.available_width()/2.0, 28.0));

            if ui.add(btn).clicked() {
                *modal = ModalTipo::Ninguna;
            };

            btn = egui::Button::new("Guardar").stroke(Stroke::NONE)
            .corner_radius(CornerRadius::same(8))
            .min_size(Vec2::new(ui.available_width(), 28.0));

            if ui.add(btn).clicked() {
                let upt_usuario = runtime.block_on(update_user(usuario_mod.clone(),contrasena.clone(),jwt_token.as_deref().unwrap_or("")));
                match upt_usuario {
                    Ok((respuesta, token)) => {
                        *mensaje = respuesta;
                        if *mensaje == "Usuario actualizado correctamente".to_string(){
                            *jwt_token = token;
                        }
                    }

                    Err(e) => {
                        *mensaje = e.to_string();
                    }
                }
                if !mensaje.is_empty(){
                    if *mensaje == "Usuario actualizado correctamente".to_string() && tiempo_exito.is_none() {
                        *tiempo_exito = Some(std::time::Instant::now());
                    }
                }
            };
        });
        if let Some(t) = tiempo_exito {
            if t.elapsed() >= std::time::Duration::from_secs(2) {
                *tiempo_exito = None;
                *modal = ModalTipo::Ninguna;
                *pantalla = Pantalla::Logout;
                *mensaje = "".to_string();
            } else {
                ui.ctx().request_repaint();
            }
        }
        if !mensaje.is_empty(){
            ui.label(
                egui::RichText::new(mensaje.clone())
                    .color(if *mensaje == "Usuario actualizado correctamente".to_string() {
                        egui::Color32::GREEN
                    } else {
                        egui::Color32::RED
                    })
                    .strong(),
            );
        }
    });
}

fn modal_correo(ui: &mut Ui, correo: &mut String, contrasena :&mut String, usuario_mod: &mut String, modal: &mut ModalTipo, jwt_token: &mut Option<String>, 
    runtime: &tokio::runtime::Runtime, mensaje: &mut String, pantalla: &mut Pantalla, tiempo_exito: &mut Option<std::time::Instant>){
    ui.add_space(4.0);
    egui::Frame::new()
    .fill(Color32::from_rgb(15, 15, 15))
    .stroke(Stroke::new(0.2, Color32::from_rgba_premultiplied(255, 255, 255, 12)))
    .corner_radius(CornerRadius::same(8))
    .inner_margin(egui::Margin::same(12))
    .show(ui, |ui|{
        ui.set_min_width(ui.available_width());
        ui.horizontal(|ui|{
            ui.vertical(|ui|{
                ui.label("Correo actual");
                ui.label(RichText::new(format!("{}", correo)).strong());
            })
        });
    });
    ui.add_space(15.0);
    ui.vertical(|ui|{
        ui.label("Correo");
        ui.add(
            //Se guarda el correo en usuario_mod
        egui::TextEdit::singleline(usuario_mod)
            .hint_text("nuevo@correo.com")
            .desired_width(INFINITY)
            .margin(egui::Margin::same(8)),
        );
        ui.add_space(10.0);
        ui.label("Contraseña actual");
        ui.add(
        egui::TextEdit::singleline(contrasena)
            .hint_text("Contraseña Actual")
            .desired_width(INFINITY)
            .margin(egui::Margin::same(8)),
        );
        ui.add_space(4.0);
        ui.label("Necesaria para confirmar el cambio.");
        ui.add_space(15.0);
        ui.horizontal(|ui|{
            let mut btn = egui::Button::new("Cancelar").stroke(Stroke::NONE)
            .corner_radius(CornerRadius::same(8))
            .min_size(Vec2::new(ui.available_width()/2.0, 28.0));

            if ui.add(btn).clicked() {
                *modal = ModalTipo::Ninguna;
            };

            btn = egui::Button::new("Guardar").stroke(Stroke::NONE)
            .corner_radius(CornerRadius::same(8))
            .min_size(Vec2::new(ui.available_width(), 28.0));

            if ui.add(btn).clicked() {
                //Se envia el correo en usuario_mod
                let upt_email = runtime.block_on(update_email(usuario_mod.clone(),contrasena.clone(),jwt_token.as_deref().unwrap_or("")));
                match upt_email {
                    Ok(respuesta) => {
                        *mensaje = respuesta;
                    }

                    Err(e) => {
                        *mensaje = e.to_string();
                    }
                }
                if !mensaje.is_empty(){
                    if *mensaje == "Correo actualizado correctamente".to_string() && tiempo_exito.is_none() {
                        *tiempo_exito = Some(std::time::Instant::now());
                    }
                }
            };
        });
        if let Some(t) = tiempo_exito {
            if t.elapsed() >= std::time::Duration::from_secs(2) {
                *tiempo_exito = None;
                *modal = ModalTipo::Ninguna;
                *pantalla = Pantalla::Logout;
                *mensaje = "".to_string();
            } else {
                ui.ctx().request_repaint();
            }
        }
        if !mensaje.is_empty(){
            ui.label(
                egui::RichText::new(mensaje.clone())
                    .color(if *mensaje == "Correo actualizado correctamente".to_string() {
                        egui::Color32::GREEN
                    } else {
                        egui::Color32::RED
                    })
                    .strong(),
            );
        }
    });
}

fn modal_contrasena(ui: &mut Ui, contrasena :&mut String, usuario_mod: &mut String, modal: &mut ModalTipo, contrasena_confirm: &mut String, jwt_token: &mut Option<String>, 
    runtime: &tokio::runtime::Runtime, mensaje: &mut String, pantalla: &mut Pantalla, tiempo_exito: &mut Option<std::time::Instant>){
    ui.add_space(4.0);
    ui.vertical(|ui|{
        ui.label("Contraseña actual");
        ui.add(
        egui::TextEdit::singleline(contrasena)
            .hint_text("*******")
            .desired_width(INFINITY)
            .margin(egui::Margin::same(8)),
        );
        ui.add_space(4.0);
        ui.label("Necesaria para confirmar el cambio.");
        ui.add_space(10.0);
        ui.label("Nueva contraseña");
        ui.add(
        egui::TextEdit::singleline(usuario_mod)
            .hint_text("Nueva contraseña")
            .desired_width(INFINITY)
            .margin(egui::Margin::same(8)),
        );
        ui.add_space(10.0);
        ui.label("Confirmacion de la contraseña");
        ui.add(
        egui::TextEdit::singleline(contrasena_confirm)
            .hint_text("Confirmacion contraseña")
            .desired_width(INFINITY)
            .margin(egui::Margin::same(8)),
        );    
        ui.add_space(15.0);
        ui.horizontal(|ui|{
            let mut btn = egui::Button::new("Cancelar").stroke(Stroke::NONE)
            .corner_radius(CornerRadius::same(8))
            .min_size(Vec2::new(ui.available_width()/2.0, 28.0));

            if ui.add(btn).clicked() {
                *modal = ModalTipo::Ninguna;
            };

            btn = egui::Button::new("Guardar").stroke(Stroke::NONE)
            .corner_radius(CornerRadius::same(8))
            .min_size(Vec2::new(ui.available_width(), 28.0));

            if ui.add(btn).clicked() {
                if usuario_mod != contrasena_confirm{
                    *mensaje = "Las contraseñas no coinciden".to_string();
                }else{
                    let upt_email = runtime.block_on(update_pass(usuario_mod.clone(),contrasena.clone(), jwt_token.as_deref().unwrap_or("")));
                    match upt_email {
                        Ok(respuesta) => {
                            *mensaje = respuesta;
                        }

                        Err(e) => {
                            *mensaje = e.to_string();
                        }
                    }
                    if !mensaje.is_empty(){
                        if *mensaje == "Contraseña actualizada correctamente".to_string() && tiempo_exito.is_none() {
                            *tiempo_exito = Some(std::time::Instant::now());
                        }
                    }
                }
            };
        });
        if let Some(t) = tiempo_exito {
            if t.elapsed() >= std::time::Duration::from_secs(2) {
                *tiempo_exito = None;
                *modal = ModalTipo::Ninguna;
                *pantalla = Pantalla::Logout;
                *mensaje = "".to_string();
            } else {
                ui.ctx().request_repaint();
            }
        }
        if !mensaje.is_empty(){
            ui.label(
                egui::RichText::new(mensaje.clone())
                    .color(if *mensaje == "Contraseña actualizada correctamente".to_string() {
                        egui::Color32::GREEN
                    } else {
                        egui::Color32::RED
                    })
                    .strong(),
            );
        }
    });
}

fn modal_eliminar(ui: &mut Ui, usuario: &mut String, contrasena :&mut String, modal: &mut ModalTipo, jwt_token: &mut Option<String>, pantalla: &mut Pantalla, 
    runtime: &tokio::runtime::Runtime, mensaje: &mut String){
    ui.add_space(4.0);
    egui::Frame::new()
    .fill(Color32::from_rgb(50, 10, 10))
    .stroke(Stroke::new(0.5, Color32::from_rgb(127, 29, 29)))
    .corner_radius(CornerRadius::same(8))
    .inner_margin(egui::Margin::same(12))
    .show(ui, |ui|{
        ui.set_min_width(ui.available_width());
        ui.horizontal(|ui|{
            ui.vertical(|ui|{
                ui.label(RichText::new("Esta acción es permanente e irreversible.").color(Color32::from_rgb(220, 80, 80)).strong());
                ui.label(RichText::new("Se eliminarán todos tus datos, incluyendo tu TMB, historial de comidas y configuración.").color(Color32::from_rgb(220, 80, 80)));
            })
        });
    });
    ui.add_space(10.0);
    egui::Frame::new()
    .fill(Color32::from_rgb(15, 15, 15))
    .stroke(Stroke::new(0.2, Color32::from_rgba_premultiplied(255, 255, 255, 12)))
    .corner_radius(CornerRadius::same(8))
    .inner_margin(egui::Margin::same(12))
    .show(ui, |ui|{
        ui.set_min_width(ui.available_width());
        ui.horizontal(|ui|{
            ui.vertical(|ui|{
                ui.label("Cuenta a eliminar");
                ui.label(RichText::new(format!("{}", usuario)).strong());
            })
        });
    });
    ui.vertical(|ui|{    
        ui.add_space(10.0);
        ui.label("Introduce tu contraseña para confirmar");
        ui.add(
        egui::TextEdit::singleline(contrasena)
            .hint_text("*****")
            .desired_width(INFINITY)
            .margin(egui::Margin::same(8)),
        );   
        ui.add_space(15.0);
        ui.horizontal(|ui|{
            let mut btn = egui::Button::new("Cancelar").stroke(Stroke::NONE)
            .corner_radius(CornerRadius::same(8))
            .min_size(Vec2::new(ui.available_width()/2.0-4.0, 34.0));

            if ui.add(btn).clicked() {
                *modal = ModalTipo::Ninguna;
            };

            btn = egui::Button::new(RichText::new("Sí, eliminar").color(Color32::from_rgb(220, 80, 80)))
            .fill(Color32::from_rgb(50, 10, 10))
            .stroke(Stroke::new(0.5, Color32::from_rgb(127, 29, 29)))
            .corner_radius(CornerRadius::same(8))
            .min_size(Vec2::new(ui.available_width(), 34.0));

            if ui.add(btn).clicked() {
                let del_user = runtime.block_on(del_user(jwt_token.as_deref().unwrap_or(""), contrasena.clone()));
                match del_user{
                    Ok(is_correct) => {
                        if is_correct{
                            *modal = ModalTipo::Ninguna;
                            *pantalla = Pantalla::Logout;
                        }else{
                            *mensaje = "Contraseña incorrecta".to_string();
                        }   
                    }

                    Err(e) => {
                        println!("{}", e.to_string());
                    }
                }
                
            };
        });
        if !mensaje.is_empty(){
            ui.label(egui::RichText::new(mensaje.clone()).color(egui::Color32::RED).strong());
        }
    });
}

async fn update_user(
    new_usuario: String,
    contrasena: String,
    token: &str
) -> Result<(String, Option<String>), String> {
    let client = reqwest::Client::new();
    let respuesta = client
        .put("http://127.0.0.1:30000/cambio-usuario")
        .header("Authorization", format!("Bearer {}", token))
        .json(&LoginRequest{
            usuario: new_usuario,
            contrasena
        })
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !respuesta.status().is_success() {
        // Antes esto hacía panic!() y cerraba toda la app de escritorio ante
        // cualquier error del backend (incluido un JWT caducado).
        let status = respuesta.status();
        let body = respuesta.text().await.unwrap_or_default();
        return Err(format!("Error backend: {} - {}", status, body));
    }

    let datos:(String, Option<String>) = respuesta.json().await.map_err(|e| e.to_string())?;
    Ok(datos)
}

async fn update_email(
    new_correo: String,
    contrasena: String,
    token: &str
) -> Result<String, String>{
    let client = reqwest::Client::new();
    let respuesta = client
        .put("http://127.0.0.1:30000/cambio-correo")
        .header("Authorization", format!("Bearer {}", token))
        .json(&UptCorreo{
            correo: new_correo,
            contrasena
        })
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !respuesta.status().is_success() {
        let status = respuesta.status();
        let body = respuesta.text().await.unwrap_or_default();
        return Err(format!("Error backend: {} - {}", status, body));
    }

    let datos:String = respuesta.text().await.map_err(|e| e.to_string())?;
    Ok(datos)
}

async fn update_pass(
    new_contrasena: String,
    ant_contrasena: String, 
    token: &str
) -> Result<String, String>{
    let client = reqwest::Client::new();
    let respuesta = client
        .put("http://127.0.0.1:30000/cambio-contrasena")
        .header("Authorization", format!("Bearer {}", token))
        .json(&UptPass{
            contrasena_ant: ant_contrasena,
            contrasena_new: new_contrasena
        })
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !respuesta.status().is_success() {
        let status = respuesta.status();
        let body = respuesta.text().await.unwrap_or_default();
        return Err(format!("Error backend: {} - {}", status, body));
    }

    let datos:String = respuesta.text().await.map_err(|e| e.to_string())?;
    Ok(datos)
}

async fn del_user(
    token: &str,
    contrasena: String
) -> Result<bool, String>{
    let client = reqwest::Client::new();
    let respuesta = client
        .delete("http://127.0.0.1:30000/delete-usuario")
        .header("Authorization", format!("Bearer {}", token))
        .body(contrasena)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !respuesta.status().is_success() {
        let status = respuesta.status();
        let body = respuesta.text().await.unwrap_or_default();
        return Err(format!("Error backend: {} - {}", status, body));
    }

    let datos:bool = respuesta.json().await.map_err(|e| e.to_string())?;
    Ok(datos)
}