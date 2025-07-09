use rustplay::MainWindow;


fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        "Binary Converter - A-Z ↔ 5-bit",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(MainWindow::default()))),
    )
}
