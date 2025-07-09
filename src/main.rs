use eframe::egui;
use std::fs;
use std::sync::Arc;

//알파벳을 2진법으로 바꿈
fn alphabet_to_bin(c: char) -> Option<String> {
    if c.is_ascii_uppercase() {
        let val = c as u8 - b'A' + 1;
        Some(format!("{:05b}", val))
    } else {
        None
    }
}

//2진법을 알파벳으로 바꿈
fn bin_to_alphabet(s: &str) -> Option<char> {
    if s.len() == 5 {
        if let Ok(val) = u8::from_str_radix(s, 2) {
            if val >= 1 && val <= 26 {
                return Some((b'A' + val - 1) as char);
            }
        }
    }
    None
}

// 이건 신경쓰지마셈 별거 아님
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

struct MyApp {
    input: String,
    output: String,
    reverse_input: String,
    reverse_output: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            input: String::new(),
            output: String::new(),
            reverse_input: String::new(),
            reverse_output: String::new(),
        }
    }
}

//앱 구현
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("알파벳 → 2진법 변환기(5-bit)");

            ui.label("입력 (알파벳만 가능):");
            if ui.text_edit_singleline(&mut self.input).changed() {
                self.output = self
                    .input
                    .chars()
                    .filter_map(|c| alphabet_to_bin(c.to_ascii_uppercase()))
                    .collect::<Vec<_>>()
                    .join(" ");
            }

            ui.separator();

            ui.label("2진법 출력:");
            ui.label(&self.output);
            ui.separator();

            ui.heading("2진법 → 알파벳 변환기");

            ui.label("입력 (5-bit 이진법 공백으로 분할, 예, 00001 00010):");
            if ui.text_edit_singleline(&mut self.reverse_input).changed() {
                self.reverse_output = self
                    .reverse_input
                    .split_whitespace()
                    .filter_map(bin_to_alphabet)
                    .collect();
            }

            ui.separator();
            ui.label("알파벳 출력:");
            ui.label(&self.reverse_output);
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "이진법 변환기 - A-Z ↔ 5-bit",
        options,
        Box::new(|cc| {
            setup_custom_fonts(&cc.egui_ctx);
            Ok(Box::new(MyApp::default()))
        }),
    )
}
