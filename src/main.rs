mod gui;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Calculadora Terminal",
        options,
        Box::new(|_cc| Box::new(gui::CalculadoraApp::default())),
    );
}