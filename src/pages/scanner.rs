use egui::{Color32, RichText, ScrollArea, Ui};
use crate::app::WifiApp;
use crate::wifi::scanner::NetworkInfo;

pub struct ScannerPage;

impl ScannerPage {
    pub fn render(app: &mut WifiApp, ui: &mut Ui) {
        ui.heading(RichText::new("📡 فحص شبكات الواي فاي").size(22.0));
        ui.separator();
        
        ui.horizontal(|ui| {
            if ui.button("🔍 بدء الفحص الشامل").clicked() {
                app.set_loading(true);
                app.set_status("جاري فحص الشبكات المحيطة...");
                app.add_log_message("بدء فحص الشبكات اللاسلكية");
                
                let scanner = app.wifi_scanner_mut();
                scanner.start_scan();
                
                app.set_loading(false);
                app.add_log_message(format!("✅ تم العثور على {} شبكة", scanner.networks().len()));
            }
            
            if ui.button("🧹 مسح النتائج").clicked() {
                app.wifi_scanner_mut().clear();
                app.add_log_message("تم مسح نتائج الفحص");
            }
            
            if ui.button("📊 تحليل القنوات").clicked() {
                app.add_log_message("تحليل ازدحام القنوات...");
                let best_channel = app.wifi_scanner().analyze_channels();
                app.add_log_message(format!("القناة الموصى بها: {}", best_channel));
            }
        });
        
        ui.separator();
        
        // عرض النتائج
        let networks = app.wifi_scanner().networks();
        
        if networks.is_empty() {
            ui.vertical_centered(|ui| {
                ui.add_space(50.0);
                ui.label(RichText::new("📭 لا توجد نتائج").size(18.0).color(Color32::GRAY));
                ui.label("اضغط على 'بدء الفحص' للبحث عن الشبكات");
            });
        } else {
            ui.label(format!("تم العثور على {} شبكة:", networks.len()));
            
            ScrollArea::vertical()
                .max_height(450.0)
                .show(ui, |ui| {
                    egui::Grid::new("networks_grid")
                        .striped(true)
                        .num_columns(6)
                        .show(ui, |ui| {
                            // رؤوس الأعمدة
                            ui.label(RichText::new("SSID").strong());
                            ui.label(RichText::new("الإشارة").strong());
                            ui.label(RichText::new("القناة").strong());
                            ui.label(RichText::new("الأمان").strong());
                            ui.label(RichText::new("BSSID").strong());
                            ui.label(RichText::new("التردد").strong());
                            ui.end_row();
                            
                            for network in networks {
                                Self::render_network_row(ui, network);
                            }
                        });
                });
        }
    }
    
    fn render_network_row(ui: &mut Ui, network: &NetworkInfo) {
        let signal_color = match network.signal_strength {
            s if s > -50 => Color32::GREEN,
            s if s > -70 => Color32::YELLOW,
            _ => Color32::RED,
        };
        
        let signal_percent = ((network.signal_strength + 100) * 2).clamp(0, 100);
        
        ui.label(&network.ssid);
        ui.label(
            RichText::new(format!("{} dBm ({}%)", network.signal_strength, signal_percent))
                .color(signal_color)
        );
        ui.label(format!("{}", network.channel));
        ui.label(&network.security);
        ui.label(&network.bssid);
        ui.label(&network.frequency);
        ui.end_row();
    }
}
