use eframe::egui::{self, Color32, FontDefinitions, FontFamily};

pub fn apply_custom_style(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();

    // Configurações de fonte
    let mut fonts = FontDefinitions::default();

    // Adicionando a fonte Open Sans
    fonts.font_data.insert("open_sans".to_owned(),
        egui::FontData::from_static(include_bytes!("Open_Sans/OpenSans-VariableFont_wdth,wght.ttf"))
    );

    // Definindo a fonte Open Sans como a primeira para fontes proporcionais
    fonts.families.get_mut(&FontFamily::Proportional).unwrap()
        .insert(0, "open_sans".to_owned());

    // Definindo a fonte Open Sans como fallback para fontes monoespaçadas
    fonts.families.get_mut(&FontFamily::Monospace).unwrap()
        .push("open_sans".to_owned());

    ctx.set_fonts(fonts);

    // Configurações de cor
    style.visuals.widgets.noninteractive.bg_fill = Color32::from_rgb(50, 50, 50); // Cor de fundo
    style.visuals.widgets.noninteractive.fg_stroke.color = Color32::from_rgb(240, 240, 240); // Cor do texto

    // Configurações de estilo para a caixa de operações
    style.visuals.widgets.inactive.bg_fill = Color32::from_rgb(255, 255, 255); // Fundo branco
    style.visuals.widgets.inactive.rounding = egui::Rounding::same(10.0); // Bordas arredondadas

    // Configurações de estilo para o botão de calcular
    style.visuals.widgets.inactive.bg_fill = Color32::from_rgb(255, 255, 255); // Fundo branco
    style.visuals.widgets.inactive.fg_stroke.color = Color32::from_rgb(0, 0, 0); // Texto preto
    style.visuals.widgets.inactive.rounding = egui::Rounding::same(10.0); // Bordas arredondadas

    style.visuals.widgets.hovered.bg_fill = Color32::from_rgb(230, 230, 230); // Fundo cinza claro
    style.visuals.widgets.hovered.fg_stroke.color = Color32::from_rgb(0, 0, 0); // Texto preto
    style.visuals.widgets.hovered.rounding = egui::Rounding::same(10.0); // Bordas arredondadas

    style.visuals.widgets.active.bg_fill = Color32::from_rgb(255, 255, 255); // Fundo branco
    style.visuals.widgets.active.fg_stroke.color = Color32::from_rgb(0, 0, 0); // Texto preto
    style.visuals.widgets.active.rounding = egui::Rounding::same(10.0); // Bordas arredondadas

    // Configurações de estilo para os inputs
    style.visuals.widgets.inactive.fg_stroke.color = Color32::from_rgb(255, 255, 255); // Texto branco nos inputs

    // Aplicar espaçamento e orientação
    style.spacing.item_spacing = egui::vec2(10.0, 10.0); // Espaçamento entre elementos
    style.spacing.button_padding = egui::vec2(10.0, 10.0); // Espaçamento interno dos botões

    ctx.set_style(style);
}