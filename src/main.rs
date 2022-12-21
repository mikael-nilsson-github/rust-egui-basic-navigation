use eframe;

use rust-egui-basic-navigation::app::switcher::SwitcherApp;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("egui navigator", native_options, Box::new(|cc| Box::new(SwitcherApp::new(cc))));
    Ok(())
}
