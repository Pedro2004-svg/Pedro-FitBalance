use crate::{Pantalla, egui};
use egui::Ui;
use egui::{Color32, FontId, RichText, CornerRadius, Stroke, Vec2};
use serde::{Deserialize, Serialize};

// ── Paleta ────────────────────────────────────────────────────────────────────
#[derive(Serialize)]
struct TMBRequest {
    usuario: String,
    calorias: String,
}

#[derive(Deserialize)]
struct TMBResponse {
    tmb: String,
    #[allow(dead_code)]
    nombre_usuario: String,
}

struct Palette;
impl Palette {
    const CARD: Color32 = Color32::from_rgb(22, 22, 22);
    const BORDER: Color32 = Color32::from_rgba_premultiplied(255, 255, 255, 18);
    const TEXT: Color32 = Color32::from_rgb(230, 230, 225);
    const MUTED: Color32 = Color32::from_rgb(120, 118, 112);
    const ACCENT: Color32 = Color32::from_rgb(59, 130, 246);
    const ACCENT_BG: Color32 = Color32::from_rgb(20, 40, 80);
    const SUCCESS: Color32 = Color32::from_rgb(52, 199, 145);
    const SUCCESS_BG: Color32 = Color32::from_rgb(10, 45, 35);
    const ERROR: Color32 = Color32::from_rgb(220, 80, 80);
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn card(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui)) {
    egui::Frame::new()
        .fill(Palette::CARD)
        .stroke(Stroke::new(0.5, Palette::BORDER))
        .corner_radius(CornerRadius::same(10))
        .inner_margin(egui::Margin::same(16))
        .show(ui, add_contents);
}

fn field_label(ui: &mut Ui, text: &str) {
    ui.label(RichText::new(text).size(12.0).color(Palette::MUTED));
    ui.add_space(4.0);
}

fn sexo_toggle(ui: &mut Ui, current: &mut String, value: &str, label: &str) {
    let selected = current.as_str() == value;
    let (bg, fg, stroke) = if selected {
        (
            Palette::ACCENT_BG,
            Palette::ACCENT,
            Stroke::new(0.5, Palette::ACCENT),
        )
    } else {
        (
            Color32::from_rgb(30, 30, 30),
            Palette::MUTED,
            Stroke::new(0.5, Palette::BORDER),
        )
    };
    let btn = egui::Button::new(RichText::new(label).size(13.0).color(fg))
        .fill(bg)
        .stroke(stroke)
        .corner_radius(CornerRadius::same(6))
        .min_size(Vec2::new(88.0, 32.0));
    if ui.add(btn).clicked() {
        *current = value.to_string();
    }
}

fn tag_badge(ui: &mut Ui, text: &str, bg: Color32, fg: Color32) {
    let galley = ui
        .painter()
        .layout_no_wrap(text.to_owned(), FontId::proportional(12.0), fg);
    let pad = Vec2::new(10.0, 4.0);
    let size = galley.size() + pad * 2.0;
    let (rect, _) = ui.allocate_exact_size(size, egui::Sense::hover());
    ui.painter().rect_filled(rect, CornerRadius::same(6), bg);
    ui.painter().galley(rect.min + pad, galley, fg);
}

// ── Vista principal ───────────────────────────────────────────────────────────

pub fn show(
    ui: &mut Ui,
    peso: &mut String,
    edad: &mut String,
    sexo: &mut String,
    altura: &mut f64,
    result: &mut String,
    _pantalla: &mut Pantalla,
    _hay_cuenta: &mut String,
    runtime: &tokio::runtime::Runtime,
    jwt_token: &Option<String>,
    tmb: &mut f64,
) -> bool {
    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.vertical_centered(|ui| {
            ui.set_max_width(600.0);

            // ── Cabecera ──────────────────────────────────────────────────
            ui.label(
                RichText::new("Tasa metabólica basal")
                    .size(20.0)
                    .strong()
                    .color(Palette::TEXT),
            );
            ui.add_space(4.0);
            ui.label(
                RichText::new(
                    "Calcula tus calorías de mantenimiento. Solo necesitas hacerlo una vez — \
                    se guarda en tu cuenta.",
                )
                .size(13.0)
                .color(Palette::MUTED),
            );
            ui.add_space(20.0);

            // ── Card formulario ───────────────────────────────────────────
            card(ui, |ui| {
                // Fila 1 — Peso + Altura
                ui.columns(2, |cols| {
                    field_label(&mut cols[0], "Peso");
                    cols[0].horizontal(|ui| {
                        ui.add(
                            egui::TextEdit::singleline(peso)
                                .hint_text("70")
                                .desired_width(ui.available_width() - 30.0)
                                .margin(egui::Margin::same(6)),
                        );
                        ui.label(RichText::new("kg").size(12.0).color(Palette::MUTED));
                    });
                    if !peso.is_empty() && peso.parse::<f64>().is_err() {
                        cols[0].label(
                            RichText::new("⚠ Número inválido")
                                .size(11.0)
                                .color(Palette::ERROR),
                        );
                    }

                    field_label(&mut cols[1], "Altura");
                    cols[1].add(
                        egui::Slider::new(altura, 100.0..=230.0)
                            .suffix(" cm")
                            .text(""),
                    );
                });

                ui.add_space(14.0);

                // Fila 2 — Edad + Sexo
                ui.columns(2, |cols| {
                    field_label(&mut cols[0], "Edad");
                    cols[0].horizontal(|ui| {
                        ui.add(
                            egui::TextEdit::singleline(edad)
                                .hint_text("28")
                                .desired_width(ui.available_width() - 40.0)
                                .margin(egui::Margin::same(6)),
                        );
                        ui.label(RichText::new("años").size(12.0).color(Palette::MUTED));
                    });
                    if !edad.is_empty() && edad.parse::<f64>().is_err() {
                        cols[0].label(
                            RichText::new("⚠ Número inválido")
                                .size(11.0)
                                .color(Palette::ERROR),
                        );
                    }

                    field_label(&mut cols[1], "Sexo");
                    cols[1].horizontal(|ui| {
                        sexo_toggle(ui, sexo, "Male", "Hombre");
                        ui.add_space(8.0);
                        sexo_toggle(ui, sexo, "Female", "Mujer");
                    });
                });

                ui.add_space(20.0);

                // ── Botón calcular ────────────────────────────────────────
                let campos_validos =
                    peso.parse::<f64>().is_ok() && edad.parse::<f64>().is_ok() && !sexo.is_empty();

                ui.add_enabled_ui(campos_validos, |ui| {
                    let btn = egui::Button::new(
                        RichText::new("Calcular TMB")
                            .size(14.0)
                            .color(Color32::WHITE),
                    )
                    .fill(if campos_validos {
                        Palette::ACCENT
                    } else {
                        Color32::from_rgb(50, 50, 50)
                    })
                    .stroke(Stroke::NONE)
                    .corner_radius(CornerRadius::same(8))
                    .min_size(Vec2::new(ui.available_width(), 38.0));

                    if ui.add(btn).clicked() {
                        match calcular_tmb(*altura, peso, edad, sexo) {
                            Ok(valor) => {
                                *result = valor.to_string();
                                // Marca que este resultado concreto aún no se
                                // ha enviado al backend. Se consume una sola
                                // vez más abajo, en vez de reenviarse en cada
                                // frame mientras la card de resultado esté
                                // visible.
                                ui.ctx().data_mut(|d| {
                                    d.insert_temp(egui::Id::new("index_tmb_pendiente_registro"), true)
                                });
                            }
                            Err(e) => *result = format!("error:{}", e),
                        }
                    }
                });
            });

            // ── Card resultado ────────────────────────────────────────────
            if !result.is_empty() && !result.starts_with("error:") {
                ui.add_space(16.0);
                card(ui, |ui| {
                    ui.label(RichText::new("Resultado").size(12.0).color(Palette::MUTED));
                    ui.add_space(12.0);

                    *tmb = result.parse().unwrap_or(0.0);
                    let tdee = *tmb * 1.55;

                    ui.columns(2, |cols| {
                        cols[0].label(RichText::new("TMB base").size(12.0).color(Palette::MUTED));
                        cols[0].add_space(2.0);
                        cols[0].label(
                            RichText::new(format!("{} kcal", tmb.ceil() as u64))
                                .size(26.0)
                                .strong()
                                .color(Palette::TEXT),
                        );

                        cols[1].label(
                            RichText::new("Con actividad (TDEE)")
                                .size(12.0)
                                .color(Palette::MUTED),
                        );
                        cols[1].add_space(2.0);
                        cols[1].label(
                            RichText::new(format!("{} kcal", tdee.ceil() as u64))
                                .size(26.0)
                                .strong()
                                .color(Palette::ACCENT),
                        );
                    });

                    ui.add_space(10.0);
                    ui.separator();
                    ui.add_space(8.0);

                    ui.horizontal(|ui| {
                        tag_badge(ui, "Mifflin-St Jeor", Palette::ACCENT_BG, Palette::ACCENT);
                        ui.add_space(6.0);
                        tag_badge(
                            ui,
                            "×1.55 moderado",
                            Color32::from_rgb(30, 30, 30),
                            Palette::MUTED,
                        );
                    });

                    ui.add_space(16.0);

                    egui::Frame::new()
                        .fill(Palette::SUCCESS_BG)
                        .corner_radius(CornerRadius::same(8))
                        .inner_margin(egui::Margin::same(12))
                        .show(ui, |ui| {
                            ui.label(
                                RichText::new("Objetivos de déficit sugeridos")
                                    .size(12.0)
                                    .color(Palette::SUCCESS),
                            );
                            ui.add_space(6.0);
                            ui.columns(2, |cols| {
                                cols[0].label(
                                    RichText::new(format!(
                                        "{} kcal/día",
                                        (tdee - 300.0).ceil() as u64
                                    ))
                                    .size(15.0)
                                    .strong()
                                    .color(Palette::SUCCESS),
                                );
                                cols[0].label(
                                    RichText::new("Déficit moderado (−300)")
                                        .size(11.0)
                                        .color(Palette::SUCCESS),
                                );
                                cols[1].label(
                                    RichText::new(format!(
                                        "{} kcal/día",
                                        (tdee - 500.0).ceil() as u64
                                    ))
                                    .size(15.0)
                                    .strong()
                                    .color(Palette::SUCCESS),
                                );
                                cols[1].label(
                                    RichText::new("Déficit agresivo (−500)")
                                        .size(11.0)
                                        .color(Palette::SUCCESS),
                                );
                            });
                        });

                    ui.add_space(14.0);
                    // Antes esto llamaba a tmb_register en CADA FRAME mientras
                    // la card de resultado estuviera visible (bloqueando la UI
                    // y machacando la API constantemente). Ahora solo se envía
                    // una vez, justo tras pulsar "Calcular TMB", consumiendo el
                    // flag que se marcó en el on-click del botón.
                    let debe_registrar = ui.ctx().data_mut(|d| {
                        let pendiente = d.get_temp_mut_or_insert_with::<bool>(
                            egui::Id::new("index_tmb_pendiente_registro"),
                            || false,
                        );
                        let estaba_pendiente = *pendiente;
                        *pendiente = false;
                        estaba_pendiente
                    });

                    if !_hay_cuenta.is_empty() && debe_registrar {
                        let _resultado =
                            runtime.block_on(tmb_register(_hay_cuenta.clone(), result.clone(), jwt_token.as_deref().unwrap_or(""),));
                    /* 
                        match resultado {
                            Ok(_) => println!("TMB registrado"),
                            Err(e) => println!("Error, {}", e),
                        } */
                    }
                });
            }

            // ── Error de cálculo ──────────────────────────────────────────
            if result.starts_with("error:") {
                ui.add_space(12.0);
                ui.label(
                    RichText::new(result.trim_start_matches("error:"))
                        .size(13.0)
                        .color(Palette::ERROR),
                );
            }

            ui.add_space(24.0);
        });
    });

    true
}

// ── Lógica de cálculo (Mifflin-St Jeor) ──────────────────────────────────────

fn calcular_tmb(
    altura: f64,
    peso: &str,
    edad: &str,
    sexo: &str
) -> Result<u64, String> {
    let peso = peso
        .parse::<f64>()
        .map_err(|_| "El peso no es un número válido".to_string())?;
    let edad = edad
        .parse::<f64>()
        .map_err(|_| "La edad no es un número válido".to_string())?;

    if peso <= 0.0 {
        return Err("El peso debe ser mayor que 0".into());
    }
    if edad <= 0.0 {
        return Err("La edad debe ser mayor que 0".into());
    }

    let resultado = match sexo {
        "Male" => (10.0 * peso) + (6.25 * altura) - (5.0 * edad) + 5.0,
        "Female" => (10.0 * peso) + (6.25 * altura) - (5.0 * edad) - 161.0,
        _ => return Err("Selecciona un sexo".into()),
    };
    Ok(resultado.ceil() as u64)
}

async fn tmb_register(usuario: String, calorias: String, token: &str) -> Result<String, String> {
    let client = reqwest::Client::builder()
    .timeout(std::time::Duration::from_secs(10))
    .build()
    .unwrap_or_else(|_| reqwest::Client::new());
    let respuesta = client
        .post("http://127.0.0.1:30000/tmb_register")
        .header("Authorization", format!("Bearer {}", token))
        .json(&TMBRequest { usuario, calorias })
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !respuesta.status().is_success() {
        // Antes esto hacía panic!() y cerraba toda la app de escritorio si la
        // API respondía con cualquier error. Ahora se propaga como un Err
        // normal para que el caller decida qué hacer (por ahora solo lo
        // imprime, pero al menos no tumba la aplicación).
        let status = respuesta.status();
        let body = respuesta.text().await.unwrap_or_default();
        return Err(format!("Error backend: {} - {}", status, body));
    }

    let datos: TMBResponse = respuesta.json().await.map_err(|e| e.to_string())?;
    Ok(datos.tmb)
}