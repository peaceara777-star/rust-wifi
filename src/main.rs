#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod pages;
mod wifi;
mod ui;

use app::WifiApp;
use eframe::NativeOptions;
use egui::ViewportBuilder;

fn main() -> eframe::Result<()> {
    env_logger::init();
    
    let options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([1100.0, 750.0])
            .with_min_inner_size([900.0, 600.0])
            .with_title("🛜 WiFi Master Toolkit - Professional Edition")
            .with_icon(
                eframe::icon_data::from_png_bytes(include_bytes!("../assets/icon.png"))
                    .unwrap_or_default()
            ),
        ..Default::default()
    };

    eframe::run_native(
        "WiFi Master Toolkit",
        options,
        Box::new(|cc| {
            let mut fonts = egui::FontDefinitions::default();
            cc.egui_ctx.set_fonts(fonts);
            Box::new(WifiApp::new(cc))
        }),
    )
}
