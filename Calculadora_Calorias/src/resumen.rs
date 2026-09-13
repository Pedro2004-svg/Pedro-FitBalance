use crate::{egui,Pantalla};
use egui::{Color32, RichText, CornerRadius, Stroke, Vec2};
use egui::Ui;
use chrono::{DateTime, Datelike, Local, Weekday};
use crate::alimentos::AlimentoConId;

// ── Paleta ────────────────────────────────────────────────────────────────────

struct Palette;
impl Palette {
    const CARD:       Color32 = Color32::from_rgb(22, 22, 22);
    const BORDER:     Color32 = Color32::from_rgba_premultiplied(255, 255, 255, 18);
    const TEXT:       Color32 = Color32::from_rgb(230, 230, 225);
    const MUTED:      Color32 = Color32::from_rgb(120, 118, 112);
    const ACCENT:     Color32 = Color32::from_rgb(59, 130, 246);
    const SUCCESS:    Color32 = Color32::from_rgb(52, 199, 145);
    const SUCCESS_BG: Color32 = Color32::from_rgb(10, 45, 35);
    const DANGER:     Color32 = Color32::from_rgb(220, 80, 80);
    const DANGER_BG:  Color32 = Color32::from_rgb(50, 10, 10);
}

// ── Helpers ───────────────────────────────────────────────────────────────────

pub fn card(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui)) {
    egui::Frame::new()
        .fill(Palette::CARD)
        .stroke(Stroke::new(0.5, Palette::BORDER))
        .corner_radius(CornerRadius::same(10))
        .inner_margin(egui::Margin::same(16))
        .show(ui, add_contents);
}

fn stat_card(ui: &mut Ui, label: &str, valor: &str, color: Color32, bg: Color32) {
    egui::Frame::new()
        .fill(bg)
        .stroke(Stroke::new(0.5, Palette::BORDER))
        .corner_radius(CornerRadius::same(10))
        .inner_margin(egui::Margin::same(16))
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            ui.label(RichText::new(label).size(12.0).color(Palette::MUTED));
            ui.add_space(4.0);
            ui.label(RichText::new(valor).size(24.0).strong().color(color));
        });
}

// ── Vista principal ───────────────────────────────────────────────────────────

pub fn show(
    ui: &mut Ui,
    tmb: &mut f64,                    // TMB guardado del usuario (0.0 si no hay)
    fecha_hoy: DateTime<Local>,             // ej. "Martes, 1 de julio de 2025"
    pantalla: &mut Pantalla,
    alimentos: &mut Vec<AlimentoConId>,
    hay_cuenta: &mut String,
    jwt_token: &Option<String>,
    runtime: &tokio::runtime::Runtime
) {
    if !hay_cuenta.is_empty(){
        // Antes esto se ejecutaba SIN CONDICIÓN en cada frame (egui repinta
        // muchas veces por segundo), bloqueando la UI y bombardeando la API
        // con una petición HTTP por frame. Ahora solo se refresca como mucho
        // una vez cada REFRESCO_SEGUNDOS, guardando el último instante en la
        // memoria de egui (persiste entre frames sin tocar el resto de la app).
        const REFRESCO_SEGUNDOS: f32 = 5.0;
        let id = egui::Id::new("resumen_tmb_ultimo_refresco");
        let ahora = std::time::Instant::now();

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
                        *tmb = valor.parse().unwrap();
                    }else{
                        *tmb = -1.0;
                    }
                    
                }
                Err(e) => {
                    println!("Error {}", e);
                }
            }
        }
    }
    let alimento_dia: Vec<_> = alimentos.iter().filter(|alimento| alimento.alimento.fecha[..10] == fecha_hoy.to_string()[..10]).collect();
    
    let mut total_kcal: f64  = alimento_dia.iter().map(|a| a.alimento.calorias as f64).sum();
    if total_kcal == -0.0{
        total_kcal = 0.0;
    }
    let balance          = total_kcal - *tmb;
    let en_deficit       = balance < 0.0;
    let hay_tmb          = *tmb > 0.0;
    

    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.vertical_centered(|ui| {
            ui.set_max_width(640.0);
            let dia_semana = match fecha_hoy.weekday(){
                Weekday::Mon => "Lunes",
                Weekday::Tue => "Martes",
                Weekday::Wed => "Miercoles",
                Weekday::Thu => "Jueves",
                Weekday::Fri => "Viernes",
                Weekday::Sat => "Sabado",
                Weekday::Sun => "Domingo",
            };
            // ── Cabecera ──────────────────────────────────────────────────
            ui.label(RichText::new(format!("{}, {}",dia_semana,fecha_hoy.format("%Y-%m-%d").to_string())).size(12.0).color(Palette::MUTED));
            ui.add_space(2.0);
            ui.label(
                RichText::new("Resumen de hoy")
                    .size(20.0)
                    .strong()
                    .color(Palette::TEXT),
            );
            ui.add_space(20.0);

            // ── Tarjetas de estadísticas ──────────────────────────────────
            ui.columns(3, |cols| {
                // TMB
                stat_card(
                    &mut cols[0],
                    "TMB",
                    &if hay_tmb {
                        format!("{:.0} kcal", tmb)
                    } else {
                        "—".to_string()
                    },
                    Palette::TEXT,
                    Palette::CARD,
                );

                // Consumidas
                stat_card(
                    &mut cols[1],
                    "Consumidas hoy",
                    &format!("{:.0} kcal", total_kcal),
                    Palette::ACCENT,
                    Palette::CARD,
                );

                // Balance
                if hay_tmb {
                    let (color, bg, signo) = if en_deficit {
                        (Palette::SUCCESS, Palette::SUCCESS_BG, "")
                    } else {
                        (Palette::DANGER, Palette::DANGER_BG, "+")
                    };
                    stat_card(
                        &mut cols[2],
                        if en_deficit { "Déficit" } else { "Superávit" },
                        &format!("{}{:.0} kcal", signo, balance),
                        color,
                        bg,
                    );
                } else {
                    stat_card(
                        &mut cols[2],
                        "Balance",
                        "Sin TMB",
                        Palette::MUTED,
                        Palette::CARD,
                    );
                }
            });

            ui.add_space(8.0);

            // Aviso si no hay TMB configurado
            if !hay_tmb {
                egui::Frame::new()
                    .fill(Color32::from_rgb(40, 30, 10))
                    .corner_radius(CornerRadius::same(8))
                    .inner_margin(egui::Margin::symmetric(12, 10))
                    .show(ui, |ui| {
                        ui.set_min_width(ui.available_width());
                        ui.horizontal(|ui| {
                            ui.label(
                                RichText::new("⚠")
                                    .size(13.0)
                                    .color(Color32::from_rgb(220, 160, 40)),
                            );
                            ui.label(
                                RichText::new(
                                    "No tienes un TMB configurado. Ve a la calculadora para calcularlo.",
                                )
                                .size(13.0)
                                .color(Color32::from_rgb(220, 160, 40)),
                            );
                        });
                    });
                ui.add_space(8.0);
            }

            ui.add_space(12.0);

            // ── Lista de alimentos ────────────────────────────────────────
            card(ui, |ui| {
                // Cabecera de la lista
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new("Comidas de hoy")
                            .size(13.0)
                            .color(Palette::MUTED),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(
                            RichText::new(format!("{} alimentos", alimento_dia.len()))
                                .size(12.0)
                                .color(Palette::MUTED),
                        );
                    });
                });

                ui.add_space(10.0);

                if alimento_dia.is_empty() {
                    // Estado vacío
                    ui.add_space(24.0);
                    ui.vertical_centered(|ui| {
                        ui.label(RichText::new("🍽").size(28.0));
                        ui.add_space(6.0);
                        ui.label(
                            RichText::new("Todavía no has registrado nada hoy")
                                .size(13.0)
                                .color(Palette::MUTED),
                        );
                    });
                    ui.add_space(24.0);
                } else {
                    // Cabecera de columnas
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new("Alimento")
                                .size(11.0)
                                .color(Palette::MUTED),
                        );
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(
                                RichText::new("kcal")
                                    .size(11.0)
                                    .color(Palette::MUTED),
                            );
                        });
                    });

                    ui.add_space(4.0);
                    ui.separator();
                    ui.add_space(2.0);

                    // Filas de alimentos
                    for (i, alimento) in alimento_dia.iter().enumerate() {
                        let _resp = ui.horizontal(|ui| {
                            ui.set_min_width(ui.available_width());

                            // Fondo al hacer hover
                            let _row_rect = ui.available_rect_before_wrap();

                            ui.label(
                                RichText::new(&alimento.alimento.nombre)
                                    .size(14.0)
                                    .color(Palette::TEXT),
                            );
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.label(
                                    RichText::new(&alimento.alimento.calorias.to_string())
                                        .size(14.0)
                                        .color(Palette::MUTED),
                                );
                            });
                        });

                        if i < alimento_dia.len() - 1 {
                            ui.add_space(2.0);
                            ui.separator();
                            ui.add_space(2.0);
                        }
                    }

                    // Fila total
                    ui.add_space(4.0);
                    ui.separator();
                    ui.add_space(6.0);
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new("Total")
                                .size(14.0)
                                .strong()
                                .color(Palette::TEXT),
                        );
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(
                                RichText::new(format!("{:.0}", total_kcal))
                                    .size(14.0)
                                    .strong()
                                    .color(Palette::ACCENT),
                            );
                        });
                    });
                }
                ui.add_space(12.0);

                // Botón añadir alimento
                let add_btn = egui::Button::new(
                    RichText::new("+ Añadir alimento")
                        .size(14.0)
                        .color(Color32::WHITE),
                )
                .fill(Palette::ACCENT)
                .stroke(Stroke::NONE)
                .corner_radius(CornerRadius::same(8))
                .min_size(Vec2::new(ui.available_width(), 38.0));

                if ui.add(add_btn).clicked() {
                    *pantalla = Pantalla::Alimentos;
                }
            });

            // ── Barra de progreso ─────────────────────────────────────────
            if hay_tmb && *tmb > 0.0 {
                ui.add_space(16.0);
                card(ui, |ui| {
                    let progreso = (total_kcal / *tmb).min(1.5) as f32;
                    let pct      = (total_kcal / *tmb * 100.0).min(150.0);

                    ui.label(
                        RichText::new("Progreso del día")
                            .size(12.0)
                            .color(Palette::MUTED),
                    );
                    ui.add_space(10.0);

                    // Barra personalizada
                    let bar_height = 10.0;
                    let (bar_rect, _) = ui.allocate_exact_size(
                        Vec2::new(ui.available_width(), bar_height),
                        egui::Sense::hover(),
                    );

                    // Fondo
                    ui.painter().rect_filled(
                        bar_rect,
                        CornerRadius::same(5),
                        Color32::from_rgb(40, 40, 40),
                    );

                    // Relleno
                    let fill_color = if progreso >= 1.0 { Palette::DANGER } else { Palette::ACCENT };
                    let fill_width = bar_rect.width() * progreso.min(1.0);
                    let fill_rect  = egui::Rect::from_min_size(
                        bar_rect.min,
                        Vec2::new(fill_width, bar_height),
                    );
                    ui.painter().rect_filled(fill_rect, CornerRadius::same(5), fill_color);

                    ui.add_space(8.0);
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new(format!("{:.0}% del TMB", pct))
                                .size(12.0)
                                .color(if progreso >= 1.0 { Palette::DANGER } else { Palette::ACCENT }),
                        );
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(
                                RichText::new(format!(
                                    "Quedan {:.0} kcal",
                                    (*tmb - total_kcal).max(0.0)
                                ))
                                .size(12.0)
                                .color(Palette::MUTED),
                            );
                        });
                    });
                });
            }

            ui.add_space(24.0);
        });
    });
}

pub async fn get_tmb(
    token: &str
) -> Result<String, reqwest::Error>{
    let client = reqwest::Client::builder()
    .timeout(std::time::Duration::from_secs(10))
    .build()
    .unwrap_or_else(|_| reqwest::Client::new());
    let respuesta = client
    .get("http://127.0.0.1:30000/get-tmb")
    .header("Authorization", format!("Bearer {}", token))
    .send()
    .await?;

    if !respuesta.status().is_success(){
        let status = respuesta.status();
        let body = respuesta.text().await.unwrap_or_default();
        println!("Error backend en get_tmb: {} - {}", status, body);
        return Ok(String::new());
    }

    let tmb:String = respuesta.json().await?;
    Ok(tmb)
}