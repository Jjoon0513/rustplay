mod utils;

use eframe::egui;
use std::fs;
use std::sync::Arc;
use eframe::egui::{Key, TextEdit, Vec2};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Hello egui!",
        options,
        Box::new(|cc| {
            setup_custom_fonts(&cc.egui_ctx);
            Ok(Box::new(MyApp::default()))
        }))
}

fn setup_custom_fonts(ctx: &egui::Context) {
    use egui::FontData;
    use egui::FontDefinitions;
    use egui::FontFamily::Proportional;

    let mut fonts = FontDefinitions::default();

    if let Ok(font_data) = fs::read("fonts/GothicA1-Bold.ttf") {
        fonts.font_data.insert(
            "my_font".to_owned(),
            Arc::from(FontData::from_owned(font_data)),
        );

        fonts
            .families
            .get_mut(&Proportional)
            .unwrap()
            .insert(0, "my_font".to_owned());

        ctx.set_fonts(fonts);
    }
}

#[derive(Default)]
struct MyApp {
    maintext: String,
    inputline: String,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let available_size = ui.available_size();
            let available_width = ui.available_width();

            let top_height = available_size.y - 30.0;
            ui.add_sized(
                egui::vec2(available_size.x, top_height),
                TextEdit::multiline(&mut self.maintext)
                    .interactive(false),
            );

            ui.add_space(5.0);

            let text_edit = TextEdit::singleline(&mut self.inputline)
                .hint_text("Type Something...")
                .min_size(Vec2::from([available_width, 0.0]));

            let response = text_edit.show(ui);

            if response.response.lost_focus() && ui.input(|i| i.key_pressed(Key::Enter)) {
                self.maintext.push_str(&format!("\n{}", self.inputline));
                self.inputline.clear();
                response.response.request_focus();
            }
        });
    }
}