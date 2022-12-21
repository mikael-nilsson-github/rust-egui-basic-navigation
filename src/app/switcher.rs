use eframe::egui;

use super::global;
use super::view_code;
use super::view_red;
use super::view_green;
use super::view_blue;

#[derive(PartialEq)]
pub enum View {
    CODE,
    RED,
    GREEN,
    BLUE,
}

pub struct State {
    view: View,
}

impl State {
    pub fn new() -> Self {
        Self {
            view: View::CODE,
        }
    }
}

pub struct SwitcherApp {
    state: State,
}

impl SwitcherApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            state: State::new(),
        }
    }
}

pub fn draw_top_panel(ui: &mut egui::Ui, frame: &mut eframe::Frame, state: &mut State) {
    egui::menu::bar(ui, |ui| {
        ui.menu_button("File", |ui| {
            if ui.button("Quit").clicked() {
                frame.close();
            }
        });
        ui.menu_button("View", |ui| {
            if ui.button("Code").clicked() {
                state.view = View::CODE;
                ui.close_menu();
            }
            if ui.button("Red").clicked() {
                state.view = View::RED;
                ui.close_menu();
            }
            if ui.button("Green").clicked() {
                state.view = View::GREEN;
                ui.close_menu();
            }
            if ui.button("Blue").clicked() {
                state.view = View::BLUE;
                ui.close_menu();
            }
        });
    });
}

pub fn draw_bottom_panel(ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
    ui.label(egui::RichText::new("BottomPanel").color(egui::Color32::WHITE));
}

impl eframe::App for SwitcherApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let panel_config = egui::containers::Frame {
            fill: global::COLOR_GRAY62,
            ..Default::default()
        };
        egui::TopBottomPanel::top("top_panel")
            .show_separator_line(false)
            .resizable(false)
            .resizable(false)
            .frame(panel_config)
            .show(ctx, |ui| {
            draw_top_panel(ui, frame, &mut self.state);
        });

        let panel_config = egui::containers::Frame {
            fill: global::COLOR_BLUE_CODE,
            ..Default::default()
        };
        egui::TopBottomPanel::bottom("bottom_panel")
            .show_separator_line(false)
            .resizable(false)
            .frame(panel_config)
            .show(ctx, |ui| {
            draw_bottom_panel(ui, frame);
        });
        
        if self.state.view == View::CODE {
            view_code::draw(ctx);
        } else if self.state.view == View::RED {
            view_red::draw(ctx); 
        } else if self.state.view == View::GREEN {
            view_green::draw(ctx); 
        } else if self.state.view == View::BLUE {
            view_blue::draw(ctx); 
        } else {
            panic!("no view found!");
        }
    }
}
