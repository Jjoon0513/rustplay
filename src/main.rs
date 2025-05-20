
mod utils;

use eframe::egui;
use std::fs;
use std::sync::Arc;
use eframe::egui::{Key, TextEdit}; //이건 냅두자

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Hello egui!",
        options,
        Box::new(|cc| {
            setup_custom_fonts(&cc.egui_ctx);
            Ok(Box::new(MyApp::default()))
        }),
    )
}

fn setup_custom_fonts(ctx: &egui::Context) {
    use egui::FontData;
    use egui::FontDefinitions;
    use egui::FontFamily::Proportional;

    let mut fonts = FontDefinitions::default();

    fonts.font_data.insert(
        "my_font".to_owned(),
        Arc::from(FontData::from_owned(
            fs::read("fonts/GothicA1-Bold.ttf").expect("폰트 파일을 읽을 수 없습니다"),
        )),
    );

    fonts
        .families
        .get_mut(&Proportional)
        .unwrap()
        .insert(0, "my_font".to_owned());

    ctx.set_fonts(fonts);
}

#[derive(Default)]
struct MyApp{
    maintext: String,
    inputline: String
}


impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let available_size = ui.available_size();

            let top_height = available_size.y - 30.0;
            ui.add_sized(
                egui::vec2(available_size.x, top_height),
                TextEdit::multiline(&mut self.maintext)
                    .code_editor()
                    .interactive(false),
            );

            ui.add_space(5.0);

            let input_id = ui.make_persistent_id("input_line");

            let response = ui.add_sized(
                [ui.available_width(), 24.0],
                TextEdit::singleline(&mut self.inputline)
                    .id_source(input_id)
                    .hint_text("Type Something..."),
            );

            if ui.memory(|mem| mem.has_focus(input_id)) && ui.input(|i| i.key_pressed(Key::Enter)) {
                self.maintext += format!("\n{}", self.inputline).as_str();
                self.inputline.clear();
            }


        });

    }
}
