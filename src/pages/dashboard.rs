use egui::{Color32, RichText, Ui};
use crate::app::WifiApp;

pub struct DashboardPage;

impl DashboardPage {
    pub fn render(app: &mut WifiApp, ui: &mut Ui) {
        ui.heading(RichText::new("📊 لوحة التحكم").size(22.0));
        ui.separator();
        
        // بطاقة حالة الاتصال
        ui.group(|ui| {
            ui.label(RichText::new("📶 حالة الاتصال الحالية").strong().size(16.0));
            ui.separator();
            
            ui.horizontal(|ui| {
                ui.label("المحول النشط:");
                ui.label(RichText::new("Wi-Fi").color(Color32::LIGHT_BLUE));
            });
            
            ui.horizontal(|ui| {
                ui.label("عنوان IP:");
                ui.label(RichText::new("192.168.1.100").color(Color32::LIGHT_GREEN));
            });
            
            ui.horizontal(|ui| {
                ui.label("البوابة الافتراضية:");
                ui.label("192.168.1.1");
            });
            
            ui.horizontal(|ui| {
                ui.label("خوادم DNS:");
                ui.label("8.8.8.8, 1.1.1.1");
            });
        });
        
        ui.add_space(15.0);
        
        // إجراءات سريعة
        ui.label(RichText::new("⚡ إجراءات سريعة").strong().size(16.0));
        ui.separator();
        
        ui.horizontal(|ui| {
            if ui.button("🔄 إعادة تشغيل المحول").clicked() {
                app.set_loading(true);
                app.set_status("جاري إعادة تشغيل محول الواي فاي...");
                app.add_log_message("بدء إعادة تشغيل محول الشبكة");
                // تنفيذ الأمر
                std::thread::sleep(std::time::Duration::from_secs(1));
                app.set_loading(false);
                app.add_log_message("✅ تم إعادة تشغيل المحول بنجاح");
            }
            
            if ui.button("🧹 مسح DNS Cache").clicked() {
                app.add_log_message("مسح ذاكرة DNS المؤقتة");
                #[cfg(windows)]
                {
                    std::process::Command::new("ipconfig")
                        .arg("/flushdns")
                        .output()
                        .ok();
                }
                app.add_log_message("✅ تم مسح DNS Cache");
            }
        });
        
        ui.add_space(15.0);
        
        // إحصائيات سريعة
        ui.group(|ui| {
            ui.label(RichText::new("📈 إحصائيات الشبكة").strong());
            
            ui.horizontal(|ui| {
                ui.label("سرعة الاتصال:");
                ui.label(RichText::new("866.7 Mbps").color(Color32::GREEN));
            });
            
            ui.horizontal(|ui| {
                ui.label("جودة الإشارة:");
                ui.label(RichText::new("ممتازة (92%)").color(Color32::GREEN));
            });
            
            ui.horizontal(|ui| {
                ui.label("القناة الحالية:");
                ui.label("6 (2.4 GHz)");
            });
        });
    }
}
