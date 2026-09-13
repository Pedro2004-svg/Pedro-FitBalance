use eframe::egui;
use eframe::egui::{Color32, CornerRadius, RichText, Stroke};
use std::sync::mpsc;
use crate::alimentos::{AlimentoConId, Editable};
use chrono::{Local, Datelike, Weekday};
use crate::alimentos::{AlimentoBBDD, Alimento};

// 1. Declaramos los módulos externos (archivos)
pub mod index;
pub mod login;
pub mod register;
pub mod resumen;
pub mod theme;
pub mod alimentos;
pub mod semanales;
pub mod perfil;

/* use crate::theme::{Palette, StrongIf}; */

// 2. Estado global de la aplicación
#[derive(PartialEq, Clone)]
pub enum Pantalla {
    Index,
    Login,
    Register,
    Resumen,
    Logout,
    Alimentos,
    Semanal,
    Perfil
}

#[derive(PartialEq)]
pub enum ModalTipo {
    Ninguna,
    Correo,
    Contrasena,
    Usuario,
    EliminarCuenta,
}

pub struct CalculadoraCalorias {
    pub peso: String,
    pub edad: String,
    pub sexo: String,
    pub altura: f64,
    pub result: String,
    pub pantalla: Pantalla,
    pub pantalla_anterior: Pantalla,
    pub usuario: String,
    pub contrasena: String,
    pub contrasena_confirm: String,
    pub correo: String,
    pub tx: mpsc::Sender<String>,
    pub rx: mpsc::Receiver<String>,
    pub resultado: Option<String>,
    pub runtime: tokio::runtime::Runtime,
    pub mensaje_completo: Option<(String, bool, String, String)>,
    pub hay_cuenta: String,
    pub tiempo_exito: Option<std::time::Instant>,
    pub tmb: f64,
    pub jwt_token: Option<String>,
    pub alimento: String,
    pub calorias_alimento: String,
    pub error: String,
    pub alimentos: Vec<AlimentoConId>,
    pub controll: bool,
    pub editado: Editable,
    pub tipo_modal: ModalTipo,
    pub correo_mod: String,
    pub mensaje:String,
    pub codigo: String
}

impl Default for CalculadoraCalorias {
    fn default() -> Self {
        let (tx, rx) = mpsc::channel();
        Self {
            tx,
            rx,
            resultado: None,
            runtime: tokio::runtime::Runtime::new().unwrap(),
            peso: String::new(),
            edad: String::new(),
            sexo: "Male".to_string(),
            altura: 170.0,
            result: String::new(),
            pantalla: Pantalla::Index,
            pantalla_anterior: Pantalla::Index,
            usuario: String::new(),
            contrasena: String::new(),
            contrasena_confirm: String::new(),
            mensaje_completo: None,
            correo: String::new(),
            hay_cuenta: String::new(),
            tiempo_exito: None,
            tmb: 0.0,
            jwt_token: None,
            alimento: String::new(),
            calorias_alimento: String::new(),
            error: String::new(),
            alimentos: Vec::new(),
            controll: false,
            editado: Editable{
                editar: false,
                nombre_modificado: "".to_string(),
                calorias_modificado: "".to_string(),
                indice: None
            },
            tipo_modal: ModalTipo::Ninguna,
            correo_mod: String::new(),
            mensaje: String::new(),
            codigo: String::new()
        }
    }
}

// 3. Lógica de renderizado (Frontend principal)
impl eframe::App for CalculadoraCalorias {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let negro = egui::Color32::from_rgb(0, 0, 0);
        let gris_muy_oscuro = egui::Color32::from_rgb(20, 20, 20);

        if !self.controll && !self.hay_cuenta.is_empty(){
            self.alimentos.clear();
            let foods = self.runtime.block_on(
                get_food(self.jwt_token.as_deref().unwrap_or(""))
            );

            match foods {
                Ok(foods) => {
                    for food in foods {
                        let dia_semana:String = match food.created_at.weekday(){
                            Weekday::Mon => "Lunes".to_string(),
                            Weekday::Tue => "Martes".to_string(),
                            Weekday::Wed => "Miercoles".to_string(),
                            Weekday::Thu => "Jueves".to_string(),
                            Weekday::Fri => "Viernes".to_string(),
                            Weekday::Sat => "Sabado".to_string(),
                            Weekday::Sun => "Domingo".to_string(),
                        };
                        self.alimentos.push(AlimentoConId{
                            alimento: Alimento {
                                nombre: food.nombre,
                                calorias: food.calorias,
                                usuario: self.hay_cuenta.to_string(),
                                fecha: food.created_at.to_string(),
                            },
                            id: food.id,
                            dia_dsemana: dia_semana.clone(),
                        });
                    }
                },
                Err(e) => {
                    println!("Error obteniendo alimentos: {}", e);
                }
            }
            self.controll = true;
        }
        
        ui.set_visuals({
            let mut v = egui::Visuals::dark(); // Usamos la base oscura
            v.panel_fill = negro; // Fondo principal de la ventana
            v.window_fill = gris_muy_oscuro; // Fondo de las tarjetas/ventanas
            v.override_text_color = Some(egui::Color32::WHITE); // Texto blanco para contraste
            v
        });
        // ── Sidebar ──
        if self.pantalla == Pantalla::Index || self.pantalla == Pantalla::Resumen || self.pantalla == Pantalla::Alimentos || self.pantalla == Pantalla::Semanal {
            egui::Panel::left("menu_lateral")
                .exact_size(180.0)
                .show_inside(ui, |ui| {
                    ui.add_space(16.0);
                    ui.horizontal(|ui| {
                        ui.add_space(12.0);
                        ui.label(RichText::new("🔥").size(20.0));
                        ui.label(
                        RichText::new("Calo")
                            .size(16.0)
                            .strong()
                    );
                    });

                    ui.add_space(20.0);
                    ui.separator();
                    ui.add_space(10.0);

                    self.nav_item(ui, Pantalla::Index, "🧮", "Calculadora TMB");
                    self.nav_item(ui, Pantalla::Resumen, "🧮", "Resumen");
                    self.nav_item(ui, Pantalla::Alimentos, "🧮", "Comidas");
                    if self.hay_cuenta.is_empty() {
                        ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                            ui.add_space(10.0);
                            self.nav_item(ui, Pantalla::Register, "📝", "Registro");
                            self.nav_item(ui, Pantalla::Login, "🔑", "Login");
                            ui.separator();
                            ui.add_space(20.0);
                        });
                    }else{
                        self.nav_item(ui, Pantalla::Semanal, "🧮", "Semanal");
                        ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                            ui.add_space(10.0);
                            self.nav_item(ui, Pantalla::Logout, "📝", "Logout");
                            self.nav_item(ui, Pantalla::Perfil, "📝", &self.hay_cuenta.clone());
                            ui.separator();
                            ui.add_space(20.0);
                        });
                    }
                });
        } else {
            egui::Panel::left("menu_lateral")
                .exact_size(180.0)
                .show_inside(ui, |ui| {
                    ui.add_space(16.0);
                    ui.horizontal(|ui| {
                        ui.add_space(12.0);
                        ui.label(RichText::new("🔥").size(20.0));
                        ui.label(
                        RichText::new("Calo")
                            .size(16.0)
                            .strong()
                            /* .color (Color32::from_rgb(30, 30, 28)), */
                    );
                    });

                    ui.add_space(20.0);
                    ui.separator();
                    ui.add_space(10.0);

                    self.nav_item(ui, Pantalla::Index, "🧮", "Calculadora TMB");
                    self.nav_item(ui, Pantalla::Resumen, "🧮", "Resumen");
                });
        }

        // ── Contenido Central (Enrutador) ──
        egui::CentralPanel::default()
            .frame(
                egui::Frame::new()
                    /* .fill(Palette::BG) */
                    .inner_margin(egui::Margin::same(24)),
            )
            .show_inside(ui, |ui| match self.pantalla {
                Pantalla::Index => {
                    if self.pantalla != self.pantalla_anterior {
                        self.mensaje_completo = None;
                        self.usuario = "".to_string();
                        self.contrasena = "".to_string();
                        self.contrasena_confirm = "".to_string();
                        self.pantalla_anterior = self.pantalla.clone();
                    }
                    index::show(
                        ui,
                        &mut self.peso,
                        &mut self.edad,
                        &mut self.sexo,
                        &mut self.altura,
                        &mut self.result,
                        &mut self.pantalla,
                        &mut self.hay_cuenta,
                        &self.runtime,
                        &self.jwt_token,
                        &mut self.tmb,
                    );
                }

                Pantalla::Alimentos => {
                    alimentos::show(
                        ui, &mut self.alimento, 
                        &mut self.calorias_alimento, 
                        self.hay_cuenta.clone(),
                        &mut self.error, 
                        &mut self.alimentos,
                        &self.runtime,
                        &self.jwt_token,
                        &mut self.controll,
                        &mut self.editado,
                        Local::now(),
                    );
                }

                Pantalla::Semanal => {
                    semanales::show(
                        ui, 
                        &mut self.alimentos,
                        self.tmb,
                        &self.jwt_token,
                        &self.runtime,
                        self.hay_cuenta.clone(),
                        &mut self.pantalla,
                        &mut self.tiempo_exito
                    );
                }

                Pantalla::Login => {
                    if self.pantalla != self.pantalla_anterior {
                        self.mensaje_completo = None;
                        self.usuario = "".to_string();
                        self.contrasena = "".to_string();
                        self.contrasena_confirm = "".to_string();
                        /* self.correo = "".to_string(); */
                        self.pantalla_anterior = self.pantalla.clone();
                    }
                    login::show(
                        ui,
                        &mut self.usuario,
                        &mut self.correo,
                        &mut self.contrasena,
                        &mut self.pantalla,
                        &self.runtime,
                        &mut self.mensaje_completo,
                        &mut self.hay_cuenta,
                        &mut self.tiempo_exito,
                        &mut self.jwt_token,
                        &mut self.codigo
                    );
                }
                Pantalla::Register => {
                    if self.pantalla != self.pantalla_anterior {
                        self.mensaje_completo = None;
                        self.usuario = "".to_string();
                        self.contrasena = "".to_string();
                        self.contrasena_confirm = "".to_string();
                        self.correo = "".to_string();
                        self.pantalla_anterior = self.pantalla.clone();
                    }
                    register::show(
                        ui,
                        &mut self.usuario,
                        &mut self.contrasena,
                        &mut self.contrasena_confirm,
                        &mut self.correo,
                        &mut self.pantalla,
                        &self.runtime,
                        &mut self.mensaje_completo,
                        &mut self.tiempo_exito,
                    );
                }

                Pantalla::Resumen => {
                    if self.pantalla != self.pantalla_anterior {
                        self.mensaje_completo = None;
                        self.usuario = "".to_string();
                        self.contrasena = "".to_string();
                        self.contrasena_confirm = "".to_string();
                        self.correo = "".to_string();
                        self.pantalla_anterior = self.pantalla.clone();
                    }
                    resumen::show(ui, &mut self.tmb, Local::now(), &mut self.pantalla, &mut self.alimentos, &mut self.hay_cuenta, &mut self.jwt_token,&self.runtime);
                }

                Pantalla::Perfil => {
                    perfil::show(ui, &mut self.hay_cuenta, &mut self.tipo_modal, &mut self.usuario, &mut self.correo, &mut self.contrasena_confirm, &mut self.contrasena,  
                        &mut self.jwt_token,&self.runtime, &mut self.mensaje, &mut self.pantalla, &mut self.tiempo_exito);
                }

                Pantalla::Logout=>{
                    self.jwt_token = None;
                    self.hay_cuenta.clear();
                    self.controll = false;
                    self.alimentos.clear();
                    self.pantalla = Pantalla::Index;
                    self.tmb = 0.0;
                }
                
            });
            
    }
}

// Componente visual local de la barra lateral
impl CalculadoraCalorias {
    fn nav_item(&mut self, ui: &mut egui::Ui, target: Pantalla, icon: &str, label: &str) {
        let active = self.pantalla == target;
        let (bg, text_color) = if active {
            (Color32::WHITE, Color32::BLACK)
        } else {
            (Color32::TRANSPARENT, Color32::RED)
        };

        egui::Frame::new()
            .fill(bg)
            .stroke(if active {
                Stroke::new(0.5, Color32::from_rgba_premultiplied(0, 0, 0, 20))
            } else {
                Stroke::NONE
            })
            .corner_radius(CornerRadius::same(8))
            .inner_margin(egui::Margin::symmetric(10, 8))
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.horizontal(|ui| {
                    ui.label(RichText::new(icon).size(16.0).color(if active {
                        text_color
                    } else {
                        Color32::WHITE
                    }));
                    ui.label(
                        RichText::new(label)
                            .size(13.0)
                            .color(if active { text_color } else { Color32::WHITE })
                            .strong(),
                    );
                });
            })
            .response
            .interact(egui::Sense::click())
            .clicked()
            .then(|| self.pantalla = target);
    }
}

// 4. Punto de entrada de la aplicación
fn main() -> eframe::Result<()> {
    let opciones = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([1000.0, 700.0])
            .with_title("Calo — Rastreador de calorías"),
        ..Default::default()
    };

    eframe::run_native(
        "Calculadora_calorias",
        opciones,
        Box::new(|cc| {
            theme::setup_fonts(&cc.egui_ctx);
            Ok(Box::new(CalculadoraCalorias::default()) as Box<dyn eframe::App>)
        }),
    )
}


pub async fn get_food(
    token: &str
) -> Result<Vec<AlimentoBBDD>, String> {
    let client = reqwest::Client::new();
    let respuesta = client
        .get("http://127.0.0.1:30000/get-alimento")
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !respuesta.status().is_success() {
        let status = respuesta.status();
        let body = respuesta.text().await.unwrap_or_default();
        return Err(format!("Error backend: {} - {}", status, body));
    }
    let datos: Vec<AlimentoBBDD> = respuesta.json().await.map_err(|e| e.to_string())?;
    Ok(datos)
}