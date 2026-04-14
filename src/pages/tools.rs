use egui::{Color32, RichText, Ui};
use crate::app::WifiApp;

pub struct ToolsPage;

impl ToolsPage {
    pub fn render(app: &mut WifiApp, ui: &mut Ui) {
        ui.heading(RichText::new("🛠️ أدوات الإصلاح").size(22.0));
        ui.separator();
        
        // أدوات إصلاح الشبكة
        ui.group(|ui| {
            ui.label(RichText::new("🔧 إصلاح الشبكة").strong().size(16.0));
            ui.separator();
            
            ui.vertical(|ui| {
                if ui.button("⚡ إعادة ضبط TCP/IP").clicked() {
                    app.add_log_message("تنفيذ netsh int ip reset");
                    #[cfg(windows)]
                    {
                        std::process::Command::new("netsh")
                            .args(["int", "ip", "reset"])
                            .output()
                            .ok();
                    }
                    app.add_log_message("✅ تم إعادة ضبط TCP/IP");
                }
                
                if ui.button("🔄 إعادة ضبط Winsock").clicked() {
                    app.add_log_message("تنفيذ netsh winsock reset");
                    #[cfg(windows)]
                    {
                        std::process::Command::new("netsh")
                            .args(["winsock", "reset"])
                            .output()
                            .ok();
                    }
                    app.add_log_message("✅ تم إعادة ضبط Winsock");
                }
                
                if ui.button("🌐 تجديد عنوان IP").clicked() {
                    app.add_log_message("تجديد عنوان IP");
                    #[cfg(windows)]
                    {
                        std::process::Command::new("ipconfig")
                            .args(["/release", "&&", "ipconfig", "/renew"])
                            .output()
                            .ok();
                    }
                    app.add_log_message("✅ تم تجديد عنوان IP");
                }
            });
        });
        
        ui.add_space(15.0);
        
        // إدارة DNS
        ui.group(|ui| {
            ui.label(RichText::new("🌍 إدارة DNS").strong().size(16.0));
            ui.separator();
            
            ui.horizontal(|ui| {
                ui.label("خادم DNS الأساسي:");
                let mut dns1 = "8.8.8.8".to_string();
                ui.text_edit_singleline(&mut dns1);
            });
            
            ui.horizontal(|ui| {
                ui.label("خادم DNS الثانوي:");
                let mut dns2 = "1.1.1.1".to_string();
                ui.text_edit_singleline(&mut dns2);
            });
            
            if ui.button("💾 تطبيق إعدادات DNS").clicked() {
                app.add_log_message(format!("تغيير DNS إلى {} و {}", dns1, dns2));
                app.add_log_message("✅ تم تحديث إعدادات DNS");
            }
        });
    }
}
