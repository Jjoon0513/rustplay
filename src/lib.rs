use eframe::egui;


fn alphabet_to_bin(c: char) -> Option<String> {
    if c.is_ascii_uppercase() {
        let val = c as u8 - b'A' + 1;
        Some(format!("{:05b}", val))
    } else {
        None
    }
}

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

pub struct MainWindow {
    input: String,
    output: String,
    reverse_input: String,
    reverse_output: String,
}

impl Default for MainWindow {
    fn default() -> Self {
        Self {
            input: String::new(),
            output: String::new(),
            reverse_input: String::new(),
            reverse_output: String::new(),
        }
    }
}

impl eframe::App for MainWindow {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Alphabet → Binary Converter (5-bit)");

            ui.label("Input (A-Z only):");
            if ui.text_edit_singleline(&mut self.input).changed() {
                self.output = self
                    .input
                    .chars()
                    .filter_map(|c| alphabet_to_bin(c.to_ascii_uppercase()))
                    .collect::<Vec<_>>()
                    .join(" ");
            }

            ui.separator();

            ui.label("Binary Output:");
            ui.label(&self.output);
            ui.separator();

            ui.heading("Binary → Alphabet Converter");

            ui.label("Input (5-bit binary, space-separated):");
            if ui.text_edit_singleline(&mut self.reverse_input).changed() {
                self.reverse_output = self
                    .reverse_input
                    .split_whitespace()
                    .filter_map(bin_to_alphabet)
                    .collect();
            }

            ui.separator();
            ui.label("Alphabet Output:");
            ui.label(&self.reverse_output);
        });
    }
}

// ===== WebAssembly 인터페이스 =====

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct WebHandle {
    runner: eframe::WebRunner,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl WebHandle {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        eframe::WebLogger::init(log::LevelFilter::Debug).ok();
        Self {
            runner: eframe::WebRunner::new(),
        }
    }

    #[wasm_bindgen]
    pub async fn start(&self, canvas: web_sys::HtmlCanvasElement) -> Result<(), wasm_bindgen::JsValue> {
        self.runner
            .start(
                canvas,
                eframe::WebOptions::default(),
                Box::new(|_cc| Ok(Box::new(MainWindow::default()))),
            )
            .await
    }

    #[wasm_bindgen]
    pub fn destroy(&self) {
        self.runner.destroy();
    }

    #[wasm_bindgen]
    pub fn has_panicked(&self) -> bool {
        self.runner.has_panicked()
    }

    #[wasm_bindgen]
    pub fn panic_message(&self) -> Option<String> {
        self.runner.panic_summary().map(|s| s.message())
    }

    #[wasm_bindgen]
    pub fn panic_callstack(&self) -> Option<String> {
        self.runner.panic_summary().map(|s| s.callstack())
    }
}
