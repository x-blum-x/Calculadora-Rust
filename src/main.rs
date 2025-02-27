mod gui;
mod style;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Calculadora Científica",
        options,
        Box::new(|_cc| Box::new(gui::CalculadoraApp::default())),
    );
}