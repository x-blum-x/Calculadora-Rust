use eframe::App;
use crate::style::apply_custom_style;

pub struct CalculadoraApp {
    numero1: String,
    numero2: String,
    operacao: String,
    resultado: String,
}

impl Default for CalculadoraApp {
    fn default() -> Self {
        Self {
            numero1: String::new(),
            numero2: String::new(),
            operacao: String::new(),
            resultado: String::new(),
        }
    }
}

impl eframe::App for CalculadoraApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        apply_custom_style(ctx); // Aplicar estilo personalizado

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Calculadora Científica");

                ui.label("Número 1:");
                let response1 = ui.text_edit_singleline(&mut self.numero1);
                if response1.changed() {
                    self.numero1.retain(|c| c.is_digit(10) || c == '.');
                }

                ui.label("Número 2:");
                let response2 = ui.text_edit_singleline(&mut self.numero2);
                if response2.changed() {
                    self.numero2.retain(|c| c.is_digit(10) || c == '.');
                }

                ui.label("Operação:");
                ui.text_edit_singleline(&mut self.operacao);

                ui.allocate_ui(ui.available_size() * 0.5, |ui| {
                    ui.group(|ui| {
                        ui.label("Operações disponíveis:");
                        ui.indent("  ", |ui| {
                            ui.label("+ (Soma)");
                            ui.label("- (Subtração)");
                            ui.label("* (Multiplicação)");
                            ui.label("/ (Divisão)");
                            ui.label("sqrt (Raiz quadrada)");
                            ui.label("cbrt (Raiz cúbica)");
                            ui.label("pow (Potenciação)");
                            ui.label("log (Logaritmo)");
                            ui.label("ln (Logaritmo natural)");
                            ui.label("exp (Exponencial)");
                        });
                    });
                });

                if ui.button("Calcular").clicked() {
                    let numero1: f64 = self.numero1.parse().unwrap_or(0.0);
                    let numero2: f64 = self.numero2.parse().unwrap_or(0.0);
                    let operacao = &self.operacao;

                    self.resultado = match operacao.as_str() {
                        "+" => (numero1 + numero2).to_string(),
                        "-" => (numero1 - numero2).to_string(),
                        "*" => (numero1 * numero2).to_string(),
                        "/" => {
                            if numero2 == 0.0 {
                                "Erro: Divisão por zero!".to_string()
                            } else {
                                (numero1 / numero2).to_string()
                            }
                        }
                        "sqrt" => numero1.sqrt().to_string(),
                        "cbrt" => numero1.cbrt().to_string(),
                        "pow" => numero1.powf(numero2).to_string(),
                        "log" => {
                            if numero2 <= 0.0 || numero2 == 1.0 {
                                "Erro: Base do logaritmo deve ser maior que 0 e diferente de 1.".to_string()
                            } else {
                                numero1.log(numero2).to_string()
                            }
                        }
                        "ln" => numero1.ln().to_string(),
                        "exp" => numero1.exp().to_string(),
                        _ => "Operação inválida!".to_string(),
                    };
                }

                ui.heading(format!("Resultado: {}", self.resultado));
            });
        });
    }
}