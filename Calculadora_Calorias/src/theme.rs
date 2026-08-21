use eframe::egui;
use eframe::egui::{Color32, FontId, RichText, CornerRadius, Stroke};

// ── Tipografía ───────────────────────────────────────────────────────────────
pub fn setup_fonts(ctx: &egui::Context) {
    let mut style = (*ctx.global_style()).clone();
    style.text_styles = [
        (egui::TextStyle::Body,      FontId::proportional(14.0)),
        (egui::TextStyle::Button,    FontId::proportional(14.0)),
        (egui::TextStyle::Heading,   FontId::proportional(18.0)),
        (egui::TextStyle::Small,     FontId::proportional(12.0)),
        (egui::TextStyle::Monospace, FontId::monospace(13.0)),
    ].into();
    ctx.set_global_style(style);
}

// ── Paleta de colores ────────────────────────────────────────────────────────
pub struct Palette;
impl Palette {
    pub const BG:         Color32 = Color32::from_rgb(245, 244, 241);
    pub const SIDEBAR:    Color32 = Color32::from_rgb(237, 236, 232);
    pub const CARD:       Color32 = Color32::from_rgb(255, 255, 255);
    pub const BORDER:     Color32 = Color32::from_rgba_premultiplied(0, 0, 0, 20);
    pub const TEXT:       Color32 = Color32::from_rgb(30, 30, 28);
    pub const MUTED:      Color32 = Color32::from_rgb(130, 128, 122);
    pub const ACCENT:     Color32 = Color32::from_rgb(23, 95, 165);
    pub const ACCENT_BG:  Color32 = Color32::from_rgb(230, 241, 251);
    pub const NAV_ACTIVE: Color32 = Color32::from_rgb(255, 255, 255);
}

// ── Helpers de UI ────────────────────────────────────────────────────────────
pub fn card(ui: &mut egui::Ui, add_contents: impl FnOnce(&mut egui::Ui)) {
    egui::Frame::new()
        .fill(Palette::CARD)
        .stroke(Stroke::new(0.5, Palette::BORDER))
        .corner_radius(CornerRadius::same(10))
        .inner_margin(egui::Margin::same(16))
        .show(ui, add_contents);
}

pub fn label_field(ui: &mut egui::Ui, text: &str) {
    ui.label(RichText::new(text).size(12.0).color(Palette::MUTED));
    ui.add_space(4.0);
}

pub trait StrongIf {
    fn strong_if(self, cond: bool) -> Self;
}
impl StrongIf for RichText {
    fn strong_if(self, cond: bool) -> Self {
        if cond { self.strong() } else { self }
    }
}