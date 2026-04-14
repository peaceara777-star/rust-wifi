use egui::{Color32, RichText, Ui};
use crate::app::WifiApp;

pub struct DiagnosticsPage;

impl DiagnosticsPage {
    pub fn render(app: &mut WifiApp, ui: &mut Ui) {
        ui.heading(RichText::new("🔧 تشخيص الأعطال").size(22.0));
        ui.separator();
        
        // اختبارات الاتصال
        ui.group(|ui| {
            ui.label(RichText::new("🌐 اختبارات الاتصال").strong().size(16.0));
            ui.separator();
            
            ui.horizontal(|ui| {
                if ui.button("اختبار Ping للإنترنت (8.8.8.8)").clicked() {
                    app.add_log_message("اختبار ping لـ 8.8.8.8");
                    let result = app.diagnostics_mut().test_ping("8.8.8.8");
                    app.add_log_message(format!("نتيجة ping: {}", result));
                }
                
                if ui.button("اختبار البوابة (192.168.1.1)").clicked() {
                    app.add_log_message("اختبار ping للبوابة");
                    let result = app.diagnostics_mut().test_ping("192.168.1.1");
                    app.add_log_message(format!("نتيجة ping للبوابة: {}", result));
                }
            });
            
            ui.horizontal(|ui| {
                if ui.button("اختبار DNS (1.1.1.1)").clicked() {
                    app.add_log_message("اختبار ping لـ Cloudflare DNS");
                    let result = app.diagnostics_mut().test_ping("1.1.1.1");
                    app.add_log_message(format!("نتيجة ping لـ DNS: {}", result));
                }
            });
        });
        
        ui.add_space(15.0);
        
        // تشخيص متقدم
        ui.group(|ui| {
            ui.label(RichText::new("🔬 تشخيص متقدم").strong().size(16.0));
            ui.separator();
            
            if ui.button("🚀 بدء التشخيص الشامل").clicked() {
                app.set_loading(true);
                app.set_status("جاري التشخيص الشامل...");
                app.add_log_message("بدء التشخيص الكامل للنظام");
                
                let report = app.diagnostics_mut().run_full_diagnostics();
                
                app.set_loading(false);
                app.add_log_message("✅ اكتمل التشخيص");
                
                // عرض النتائج
                ui.label(RichText::new("نتائج التشخيص:").strong());
                for (test, result) in report.tests {
                    let color = if result.passed { Color32::GREEN } else { Color32::RED };
                    ui.label(
                        RichText::new(format!("{}: {}", test, result.message))
                            .color(color)
                    );
                }
            }
        });
        
        ui.add_space(15.0);
        
        // معلومات النظام
        ui.collapsing("📊 معلومات النظام", |ui| {
            let sys_info = app.diagnostics().get_system_info();
            
            ui.label(format!("نظام التشغيل: {}", sys_info.os));
            ui.label(format!("محول الشبكة: {}", sys_info.adapter));
            ui.label(format!("MAC Address: {}", sys_info.mac));
            ui.label(format!("سرعة الارتباط: {}", sys_info.link_speed));
        });
    }
}
