use crate::alimentos::AlimentoConId;
use egui::{Ui,Color32, RichText};
use crate::{egui, Pantalla};
use crate::resumen::{card, get_tmb};
use chrono::{Datelike, Local, NaiveDate, Duration, Weekday};

pub fn show(
    ui: &mut Ui, 
    alimentos: &mut Vec<AlimentoConId>,
    mut tmb: f64,
    jwt_token: &Option<String>,
    runtime: &tokio::runtime::Runtime,
    hay_cuenta: String,
    pantalla: &mut Pantalla,
    tiempo_exito: &mut Option<std::time::Instant>
){
    if hay_cuenta.is_empty(){
        if tiempo_exito.is_none(){
            *tiempo_exito = Some(std::time::Instant::now());
        }
        ui.allocate_ui_with_layout(ui.available_size(), egui::Layout::centered_and_justified(egui::Direction::TopDown), |ui|{
            ui.label(RichText::new("Para entrar en esta seccion debe de iniciar sesión, posteriormente").size(24.0).color(Color32::from_rgb(220, 80, 80)));
        });
        if let Some(t) = tiempo_exito {
            if t.elapsed() >= std::time::Duration::from_secs(2) {
                *tiempo_exito = None;
                *pantalla = Pantalla::Login;
            } else {
                ui.ctx().request_repaint();
            }
        }
        return; 
    }

    let fecha_hoy = Local::now().date_naive();
    let dia_semana = fecha_hoy.weekday().num_days_from_monday();
    let lunes = fecha_hoy - Duration::days(dia_semana as i64);
    let fechas_semana: Vec<NaiveDate> = (0..7).map(|i| lunes + Duration::days(i)).collect();
    let id = egui::Id::new("resumen_tmb_ultimo_refresco");
    let ahora = std::time::Instant::now();
    const REFRESCO_SEGUNDOS: f32 = 5.0;
    let debe_refrescar = ui.ctx().data_mut(|d| {
        let ultimo = d.get_temp_mut_or_insert_with::<Option<std::time::Instant>>(id, || None);
        let toca = match *ultimo {
            None => true,
            Some(t) => ahora.duration_since(t).as_secs_f32() >= REFRESCO_SEGUNDOS,
        };
        if toca {
            *ultimo = Some(ahora);
        }
        toca
    });

    if debe_refrescar {
        let datos = runtime.block_on(get_tmb(jwt_token.as_deref().unwrap_or("")));
        match datos{
            Ok(valor) => {
                if !valor.is_empty(){
                    tmb = valor.parse().unwrap();
                }else{
                    tmb = -1.0;
                }
                
            }
            Err(e) => {
                println!("Error {}", e);
            }
        }
    }
    egui::ScrollArea::vertical().show(ui, |ui|{ 
        ui.vertical_centered(|ui|{
            for fecha in fechas_semana {
                let dia = match fecha.weekday(){
                    Weekday::Mon => "Lunes",
                    Weekday::Tue => "Martes",
                    Weekday::Wed => "Miercoles",
                    Weekday::Thu => "Jueves",
                    Weekday::Fri => "Viernes",
                    Weekday::Sat => "Sabado",
                    Weekday::Sun => "Domingo",
                };

                card(ui,|ui|{
                    let alimento_dia: Vec<_> = alimentos.iter().filter(|alimento| alimento.dia_dsemana == dia.to_string()).collect();
                    let mut total_kcal: f64  = alimento_dia.iter().map(|a| a.alimento.calorias as f64).sum();
                    if total_kcal == -0.0{
                        total_kcal = 0.0;
                    }
                    let hay_tmb= tmb > 0.0;
                    ui.label(RichText::new(format!("{}, {}", dia , fecha.to_string())).size(12.0).color(Color32::from_rgb(120, 118, 112)));
                    ui.add_space(4.0);
                    ui.separator();
                    ui.add_space(12.0);
                    if alimento_dia.is_empty(){
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

                        for (i, alimento) in alimento_dia.iter().enumerate(){
                            let _resp = ui.horizontal(|ui| {
                                ui.set_min_width(ui.available_width());

                                // Fondo al hacer hover
                                let _row_rect = ui.available_rect_before_wrap();

                                ui.label(
                                    RichText::new(&alimento.alimento.nombre)
                                        .size(14.0)
                                        .color(Color32::from_rgb(230, 230, 225)),
                                );
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    ui.label(
                                        RichText::new(&alimento.alimento.calorias.to_string())
                                            .size(14.0)
                                            .color(Color32::from_rgb(120, 118, 112)),
                                    );
                                });
                            });

                            if i < alimento_dia.len() - 1 {
                                ui.add_space(2.0);
                                ui.separator();
                                ui.add_space(2.0);
                            }
                        }
                        ui.add_space(4.0);
                        ui.separator();
                        ui.add_space(6.0);
                        ui.horizontal(|ui| {
                            ui.label(
                                RichText::new("Total")
                                    .size(14.0)
                                    .strong()
                                    .color(Color32::from_rgb(230, 230, 225)),
                            );
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.label(
                                    RichText::new(format!("{:.0}", total_kcal))
                                        .size(14.0)
                                        .strong()
                                        .color(Color32::from_rgb(59, 130, 246)),
                                );
                            });
                        });
                        ui.add_space(4.0);
                        ui.separator();
                        ui.add_space(6.0);
                        if hay_tmb{
                            let balance= total_kcal - tmb;
                            ui.horizontal(|ui| {
                                ui.label(
                                    RichText::new("Deficit Colorico")
                                        .size(14.0)
                                        .strong()
                                        .color(Color32::from_rgb(230, 230, 225)),
                                );
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    if balance < -0.0{
                                        ui.label(
                                            RichText::new(format!("{:.0}", balance))
                                                .size(14.0)
                                                .strong()
                                                .color(Color32::from_rgb(52, 199, 145)),
                                        );
                                    }else{
                                        ui.label(
                                            RichText::new(format!("{:.0}", balance))
                                                .size(14.0)
                                                .strong()
                                                .color(Color32::from_rgb(220, 80, 80)),
                                        );
                                    }
                                });
                            });
                        }
                    }
                });
                ui.add_space(12.0);
            }
        });
    });
}