use egui::{Color32, RichText, Ui};
use crate::app::WifiApp;

pub struct AdvancedPage;

impl AdvancedPage {
    pub fn render(app: &mut WifiApp, ui: &mut Ui) {
        ui.heading(RichText::new("⚙️ إعدادات متقدمة").size(22.0));
        ui.separator();
        
        ui.group(|ui| {
            ui.label(RichText::new("🎨 إعدادات الواجهة").strong().size(16.0));
            ui.separator();
            
            let mut dark_mode = true;
            ui.checkbox(&mut dark_mode, "الوضع الليلي");
            
            let mut rtl = true;
            ui.checkbox(&mut rtl, "دعم اللغة العربية (RTL)");
        });
        
        ui.add_space(15.0);
        
        ui.group(|ui| {
            ui.label(RichText::new("📁 إدارة البيانات").strong().size(16.0));
            ui.separator();
            
            if ui.button("📄 تصدير تقرير التشخيص").clicked() {
                app.add_log_message("جاري تصدير التقرير...");
                
                let report = serde_json::json!({
                    "timestamp": chrono::Local::now().to_rfc3339(),
                    "networks": app.wifi_scanner().networks().len(),
                    "logs": app.log_messages,
                });
                
                if let Ok(json) = serde_json::to_string_pretty(&report) {
                    std::fs::write("wifi_report.json", json).ok();
                    app.add_log_message("✅ تم تصدير التقرير إلى wifi_report.json");
                }
            }
            
            if ui.button("🗑️ مسح البيانات المؤقتة").clicked() {
                app.add_log_message("مسح البيانات المؤقتة...");
                app.wifi_scanner_mut().clear();
                app.add_log_message("✅ تم مسح البيانات المؤقتة");
            }
        });
        
        ui.add_space(15.0);
        
        ui.collapsing("ℹ️ حول البرنامج", |ui| {
            ui.label(RichText::new("WiFi Master Toolkit").size(18.0).strong());
            ui.label("الإصدار: 1.0.0");
            ui.label("مطور بـ: Rust 🦀");
            ui.label("مكتبة الواجهة: egui");
            ui.separator();
            ui.label("أداة احترافية لتشخيص وإصلاح مشاكل الواي فاي");
            ui.hyperlink_to("GitHub Repository", "https://github.com/yourusername/wifi_master_toolkit");
        });
    }
}
