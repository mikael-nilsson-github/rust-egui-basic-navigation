use eframe::egui;
use super::global;

pub fn draw_side_panel_left(ctx: &egui::Context) {
    let panel_config = egui::containers::Frame {
        fill: global::COLOR_RED1,
        ..Default::default()
    };
    egui::SidePanel::left("left_panel")
        .show_separator_line(false)
        .resizable(false)
        .frame(panel_config)
        .show(ctx, |ui| {
        ui.label("SidePanelLeft");
    });
}

pub fn draw_side_panel_right(ctx: &egui::Context) {
    let panel_config = egui::containers::Frame {
        fill: global::COLOR_RED2,
        ..Default::default()
    };
    egui::SidePanel::right("right_panel")
        .show_separator_line(false)
        .resizable(false)
        .frame(panel_config)
        .show(ctx, |ui| {
        ui.label("SidePanelRight");
    });
}

pub fn draw_central_panel(ctx: &egui::Context) {
    let panel_config = egui::containers::Frame {
        fill: global::COLOR_RED3,
        ..Default::default()
    };
    egui::CentralPanel::default()
        .frame(panel_config)
        .show(ctx, |ui| {
        ui.label("CentralPanel");
    });
}

pub fn draw(ctx: &egui::Context) {
    draw_side_panel_left(ctx);
    draw_side_panel_right(ctx);
    draw_central_panel(ctx);
}
