use crate::{egui};
use egui::{Color32, CornerRadius, RichText, Stroke, Ui, Vec2};
use serde::{Deserialize,Serialize};
use chrono::{DateTime, Datelike, Local, Weekday};
use crate::resumen::card;

#[derive(Deserialize)]
pub struct AlimentoResponse {
    pub mensaje_alimento: String
}

#[derive(Deserialize)]
pub struct AlimentoBBDD {
    pub nombre:   String,
    pub calorias: i64,
    pub id: i64,
    pub created_at: DateTime<Local>
}

#[derive(Serialize)]
pub struct AlimentoUpt {
    pub nombre:   String,
    pub calorias: i64,
    pub id: i64
}

#[derive(Serialize, Clone)]
pub struct Alimento {
    pub nombre:   String,
    pub calorias: i64,
    pub usuario: String,
    pub fecha: String
}

#[derive(Serialize, Clone)]
pub struct AlimentoConId{
    pub alimento: Alimento,
    pub id: i64,
    pub dia_dsemana: String
}

pub struct Editable {
    pub editar: bool,
    pub nombre_modificado: String,
    pub calorias_modificado: String,
    pub indice: Option<usize>
}

pub fn show(
    ui: &mut Ui, 
    alimento_nombre: &mut String, 
    calorias: &mut String, 
    hay_cuenta: String, 
    error: &mut String, 
    alimentos: &mut Vec<AlimentoConId>, 
    runtime: &tokio::runtime::Runtime,
    jwt_token: &Option<String>,
    controll: &mut bool,
    editado: &mut Editable,
    fecha_hoy: DateTime<Local>
    ) {
    let mut alimento_dia: Vec<_> = alimentos
        .iter()
        .filter(|alimento| alimento.alimento.fecha[..10] == fecha_hoy.to_string()[..10])
        .cloned()
        .collect();

    let mut total_kcal: f64 = alimento_dia
        .iter()
        .map(|a| a.alimento.calorias as f64)
        .sum();
    if total_kcal == -0.0 {
        total_kcal = 0.0;
    }
    let dia_semana:String = match fecha_hoy.weekday(){
        Weekday::Mon => "Lunes".to_string(),
        Weekday::Tue => "Martes".to_string(),
        Weekday::Wed => "Miercoles".to_string(),
        Weekday::Thu => "Jueves".to_string(),
        Weekday::Fri => "Viernes".to_string(),
        Weekday::Sat => "Sabado".to_string(),
        Weekday::Sun => "Domingo".to_string(),
    };

    egui::ScrollArea::vertical().show(ui, |ui|{
        ui.vertical_centered(|ui|{
            ui.set_max_width(640.0);
            ui.label(RichText::new("Registro de Alimentos").color(Color32::from_rgb(230, 230, 225)).size(20.0).strong());
            ui.add_space(4.0);
            ui.label(RichText::new("Añade lo que has comido hoy. Si el alimento esta en nuestra Base de Datos se rellenaran las calorias automaticamente").size(13.0).color(Color32::from_rgb(120, 118, 112)));
            ui.add_space(20.0);
            
            card(ui, |ui| {
                ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui|{
                    ui.label(RichText::new("Nombre del alimento o receta").size(12.0).color(Color32::from_rgb(120, 118, 112)));
                    ui.add_space(4.0);
                    ui.add(
                        egui::TextEdit::singleline(alimento_nombre)
                            .hint_text("ej. Pollo a la plancha, Tortilla...")
                            .desired_width(f32::INFINITY)
                            .margin(egui::Margin::same(8)),
                    );
                    ui.vertical_centered(|ui| {
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                ui.add_space(14.0);
                                ui.label(RichText::new("Calorias").size(12.0).color(Color32::from_rgb(120, 118, 112)));
                                ui.add_space(4.0);
                                ui.horizontal(|ui| {
                                    ui.set_max_width((ui.available_width() - 4.0)/2.0);
                                    ui.add(
                                        egui::TextEdit::singleline(calorias)
                                            .hint_text("0")
                                            .desired_width(f32::INFINITY)
                                            .margin(egui::Margin::same(8)),
                                    );
                                    ui.label(RichText::new("kcal").color(Color32::from_rgb(120, 118, 112)));
                                });
                            });
                        });
                        ui.add_space(14.0);
                        let add_btn = egui::Button::new(
                        RichText::new("+ Añadir alimento")
                            .size(14.0)
                            .color(Color32::WHITE),
                        )
                        .fill(Color32::from_rgb(59, 130, 246))
                        .stroke(Stroke::NONE)
                        .corner_radius(CornerRadius::same(8))
                        .min_size(Vec2::new(ui.available_width(), 38.0));

                        if ui.add(add_btn).clicked() {
                            if calorias.is_empty() || alimento_nombre.is_empty() {
                                *error = "Introduce los parametros".to_string();
                            }else{
                                //Si hay cuenta se registra el alimento en la BBDD si no se introduce en el array
                                if !hay_cuenta.is_empty() {
                                    let _resultado = runtime.block_on(
                                        register_food(calorias.parse::<i64>().unwrap(),alimento_nombre.to_string(), hay_cuenta.to_string(), fecha_hoy, jwt_token.as_deref().unwrap_or("")));
                                    *controll = false;
                                }else{
                                    let alimento_introducido =
                                        Alimento{
                                            nombre: alimento_nombre.clone(),
                                            calorias: calorias.parse::<i64>().unwrap(),
                                            usuario: hay_cuenta.to_string(),
                                            fecha: fecha_hoy.to_string() 
                                        };

                                    alimentos.push(AlimentoConId { alimento: alimento_introducido, id: 0, dia_dsemana: dia_semana.clone()});
                                }
                                //Borra los campos del los inputs al presionar el boton de añadir alimento si todos los campos estan introducidos correctamente
                                *alimento_nombre = "".to_string();
                                *calorias = "".to_string();
                            }
                        }
                        
                        if !error.is_empty(){
                            ui.label(error.clone());
                        }
                    });
                });
            });
            ui.add_space(14.0);
            card(ui, |ui|{
                ui.vertical_centered(|ui|{
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new("Comidas de hoy")
                                .size(13.0)
                                .color(Color32::from_rgb(120, 118, 112)),
                        );
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(
                                RichText::new(format!("{} alimentos", alimento_dia.len()))
                                    .size(12.0)
                                    .color(Color32::from_rgb(120, 118, 112)),
                            );
                        });
                    });
                    ui.add_space(10.0);
                    ui.horizontal(|ui|{
                        ui.label(RichText::new("Alimento")
                            .size(12.0)
                            .color(Color32::from_rgb(120, 118, 112)));
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui|{
                            ui.label(RichText::new("kcal").size(12.0)
                            .color(Color32::from_rgb(120, 118, 112)));
                        });
                    });
                    ui.add_space(4.0);
                    ui.separator();
                    ui.add_space(12.0);
                    //Muestra por pantalla los Alimentos que esten introducidos en el array 
                    if alimento_dia.is_empty(){
                        // Estado vacío
                        ui.add_space(24.0);
                        ui.vertical_centered(|ui| {
                            ui.label(RichText::new("🍽").size(28.0));
                            ui.add_space(6.0);
                            ui.label(
                                RichText::new("Todavía no has registrado nada hoy")
                                    .size(13.0)
                                    .color(Color32::from_rgb(120, 118, 112)),
                            );
                        });
                        ui.add_space(24.0);
                    }else{
                        ui.add_space(12.0);
                        // Filas de alimentos
                        let mut eliminar: Option<usize> = None;
                        let len = alimento_dia.len();
                        for (i, alimento) in alimento_dia.iter_mut().enumerate() {
                            let _resp = ui.horizontal(|ui| {
                                ui.set_min_width(ui.available_width());
                                // Fondo al hacer hover
                                let _row_rect = ui.available_rect_before_wrap();
                                if Some(i) == editado.indice && editado.editar {
                                    ui.add(
                                    egui::TextEdit::singleline(&mut editado.nombre_modificado)
                                        .hint_text(&alimento.alimento.nombre)
                                        .desired_width(150.0)
                                        .margin(egui::Margin::same(8)),
                                    );

                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                        // Botón Cancelar
                                        let cnl_btn = egui::Button::new(
                                            RichText::new("❌").size(11.0).color(Color32::from_rgb(120, 118, 112)),
                                        )
                                        .fill(Color32::TRANSPARENT)
                                        .stroke(Stroke::NONE)
                                        .min_size(Vec2::new(20.0, 20.0));

                                        // Botón Confirmar
                                        let cnf_btn = egui::Button::new(
                                            RichText::new("✅").size(11.0).color(Color32::from_rgb(0, 255, 0)),
                                        )
                                        .fill(Color32::TRANSPARENT)
                                        .stroke(Stroke::NONE)
                                        .min_size(Vec2::new(20.0, 20.0));

                                        if ui.add(cnf_btn).clicked(){
                                            for foods in alimentos.iter_mut() {
                                                if foods.alimento.nombre == alimento.alimento.nombre{
                                                    foods.alimento.nombre = editado.nombre_modificado.clone();
                                                }

                                                if foods.alimento.calorias == alimento.alimento.calorias{
                                                    foods.alimento.calorias = editado.calorias_modificado.parse::<i64>().unwrap();
                                                }
                                            }
                                            
                                            if !hay_cuenta.is_empty(){
                                                //Funcion API que actualiza la BBDD o el array de alimentos
                                                /* alimento.alimento.nombre = editado.nombre_modificado.clone();
                                                alimento.alimento.calorias = editado.calorias_modificado.parse::<i64>().unwrap(); */
                                                let _update = runtime.block_on(update_food(alimento.id, editado.calorias_modificado.parse::<i64>().unwrap(), editado.nombre_modificado.clone(), jwt_token.as_deref().unwrap_or("")));
                                            }

                                            editado.editar = false;
                                        }

                                        if ui.add(cnl_btn).clicked() {
                                            editado.editar = false;
                                        }

                                        ui.add_space(8.0);
                                        ui.add(
                                    egui::TextEdit::singleline(&mut editado.calorias_modificado)
                                        .hint_text(&alimento.alimento.calorias.to_string())
                                        .desired_width(150.0)
                                        .margin(egui::Margin::same(8)),
                                        );
                                    });
                                }else{
                                    ui.label(
                                        RichText::new(&alimento.alimento.nombre.clone())
                                            .size(14.0)
                                            .color(Color32::from_rgb(230, 230, 225)),
                                    );
                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                        // Botón eliminar
                                        let del_btn = egui::Button::new(
                                            RichText::new("\u{1F5D1}").size(11.0).color(Color32::from_rgb(120, 118, 112)),
                                        )
                                        .fill(Color32::TRANSPARENT)
                                        .stroke(Stroke::NONE)
                                        .min_size(Vec2::new(20.0, 20.0));

                                        let edit_btn = egui::Button::new(
                                            RichText::new("\u{270F}").size(11.0).color(Color32::from_rgb(0, 255, 0)),
                                        )
                                        .fill(Color32::TRANSPARENT)
                                        .stroke(Stroke::NONE)
                                        .min_size(Vec2::new(20.0, 20.0));

                                        if ui.add(edit_btn).clicked(){
                                            editado.editar = true;
                                            editado.nombre_modificado = alimento.alimento.nombre.clone();
                                            editado.calorias_modificado = alimento.alimento.calorias.to_string();
                                            editado.indice = Some(i);
                                        }

                                        if ui.add(del_btn).clicked() {
                                            if !hay_cuenta.is_empty(){
                                                let _foods = runtime.block_on(
                                            delete_food(alimento.id, jwt_token.as_deref().unwrap_or(""))
                                                );
                                            }

                                            let pos = alimentos.iter().position(|food| food.id == alimento.id);
                                            eliminar = pos;
                                        }

                                        ui.add_space(8.0);
                                        ui.label(
                                            RichText::new(&alimento.alimento.calorias.to_string())
                                                .size(14.0)
                                                .color(Color32::from_rgb(120, 118, 112)),
                                        );
                                    });
                                }
                            });
                            
                            if i < len - 1 {
                                ui.add_space(2.0);
                                ui.separator();
                                ui.add_space(2.0);
                            }
                        }

                        if let Some(idx) = eliminar {
                            alimentos.remove(idx);
                        }

                        // Fila total
                        ui.add_space(4.0);
                        ui.separator();
                        ui.add_space(6.0);
                    }
                    ui.add_space(24.0);
                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new("Total")
                                .size(14.0)
                                .strong()
                                .color(Color32::from_rgb(230, 230, 225)),
                        );
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(
                                RichText::new(format!("{:.0} kcal", total_kcal))
                                    .size(14.0)
                                    .strong()
                                    .color(Color32::from_rgb(59, 130, 246)),
                            );
                        });
                    });
                });
            });
        });
    });
}


async fn register_food(
    calorias: i64,
    nombre: String,
    usuario: String,
    fecha_hoy: DateTime<Local>,
    token: &str
) -> Result<AlimentoResponse, reqwest::Error> {

let fecha = fecha_hoy.to_string();
let alimento = Alimento {
    nombre,
    calorias,
    usuario,
    fecha
};
    let client = reqwest::Client::new();
    let respuesta = client
        .post("http://127.0.0.1:30000/alimento_register")
        .header("Authorization", format!("Bearer {}", token))
        .json(&alimento)
        .send()
        .await?;

    if !respuesta.status().is_success() {
        let status = respuesta.status();
        let body = respuesta.text().await?;
        panic!("Error backend: {} - {}", status, body);
    }
    let datos: AlimentoResponse = respuesta.json().await?;
    Ok(datos)
}

async fn delete_food(
    id: i64,
    token: &str
) -> Result<String, reqwest::Error> {
    println!("Entro en el delete del front");
    let client = reqwest::Client::new();
    let respuesta = client
        .delete("http://127.0.0.1:30000/delete-alimento")
        .header("Authorization", format!("Bearer {}", token))
        .body(id.to_string())
        .send()
        .await?;

    if !respuesta.status().is_success() {
        let status = respuesta.status();
        let body = respuesta.text().await?;
        panic!("Error backend: {} - {}", status, body);
    }
    let datos: String = respuesta.json().await?;
    Ok(datos)
}

async fn update_food(
    id: i64,
    calorias: i64,
    nombre: String,
    token: &str
) -> Result<AlimentoBBDD, reqwest::Error> {
    
    let alimento = AlimentoUpt {
        nombre,
        calorias,
        id
    };

    let client = reqwest::Client::new();
    let respuesta = client
        .put("http://127.0.0.1:30000/actualizar-alimento")
        .header("Authorization", format!("Bearer {}", token))
        .json(&alimento)
        .send()
        .await?;

    if !respuesta.status().is_success() {
        let status = respuesta.status();
        let body = respuesta.text().await?;
        panic!("Error backend: {} - {}", status, body);
    }
    let datos: AlimentoBBDD = respuesta.json().await?;
    Ok(datos)
}