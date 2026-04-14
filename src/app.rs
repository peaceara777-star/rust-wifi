use eframe::{egui, Frame};
use egui::{Align2, Color32, Context, ScrollArea, Ui};
use log::{error, info, warn};

use crate::pages::{dashboard::DashboardPage, scanner::ScannerPage, diagnostics::DiagnosticsPage, tools::ToolsPage, advanced::AdvancedPage};
use crate::ui::theme::AppTheme;
use crate::wifi::scanner::WifiScanner;
use crate::wifi::diagnostics::DiagnosticsEngine;

#[derive(Clone, PartialEq)]
pub enum Page {
    Dashboard,
    Scanner,
    Diagnostics,
    Tools,
    Advanced,
}

impl Page {
    fn name(&self) -> &'static str {
        match self {
            Page::Dashboard => "📊 لوحة التحكم",
            Page::Scanner => "📡 فحص الشبكات",
            Page::Diagnostics => "🔧 تشخيص الأعطال",
            Page::Tools => "🛠️ أدوات الإصلاح",
            Page::Advanced => "⚙️ إعدادات متقدمة",
        }
    }
    
    fn icon(&self) -> &'static str {
        match self {
            Page::Dashboard => "📊",
            Page::Scanner => "📡",
            Page::Diagnostics => "🔧",
            Page::Tools => "🛠️",
            Page::Advanced => "⚙️",
        }
    }
}

pub struct WifiApp {
    current_page: Page,
    theme: AppTheme,
    wifi_scanner: WifiScanner,
    diagnostics: DiagnosticsEngine,
    status_message: String,
    loading: bool,
    log_messages: Vec<String>,
}

impl WifiApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let theme = AppTheme::default();
        cc.egui_ctx.set_visuals(theme.get_visuals());
        
        Self {
            current_page: Page::Dashboard,
            theme,
            wifi_scanner: WifiScanner::new(),
            diagnostics: DiagnosticsEngine::new(),
            status_message: "جاهز".to_string(),
            loading: false,
            log_messages: vec!["🚀 تم بدء تشغيل مجموعة أدوات الواي فاي".to_string()],
        }
    }
    
    fn add_log(&mut self, msg: impl Into<String>) {
        let msg = msg.into();
        info!("{}", msg);
        self.log_messages.push(msg);
        if self.log_messages.len() > 50 {
            self.log_messages.remove(0);
        }
    }
}

impl eframe::App for WifiApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        // تطبيق السمة
        ctx.set_visuals(self.theme.get_visuals());
        
        // تصميم RTL للعربية
        ctx.set_pixels_per_point(1.2);
        
        egui::CentralPanel::default().show(ctx, |ui| {
            self.render_header(ui);
            ui.separator();
            self.render_layout(ui);
        });
        
        // نافذة التحميل
        if self.loading {
            egui::Window::new("⏳ جاري التنفيذ...")
                .collapsible(false)
                .resizable(false)
                .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add(egui::Spinner::new().size(50.0));
                        ui.add_space(15.0);
                        ui.label(egui::RichText::new(&self.status_message).size(16.0));
                    });
                });
        }
    }
}

impl WifiApp {
    fn render_header(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.heading(
                egui::RichText::new("🛜 مجموعة أدوات الواي فاي الاحترافية")
                    .size(26.0)
                    .strong()
                    .color(Color32::from_rgb(100, 180, 255)),
            );
            
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("🌙").on_hover_text("تبديل المظهر").clicked() {
                    self.theme.toggle();
                    ui.ctx().set_visuals(self.theme.get_visuals());
                }
                
                if ui.button("ℹ️").on_hover_text("حول البرنامج").clicked() {
                    self.add_log("عرض معلومات البرنامج");
                }
            });
        });
        
        // شريط الحالة
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("📌 الحالة:").small());
            ui.label(egui::RichText::new(&self.status_message).small().color(Color32::LIGHT_GRAY));
        });
    }
    
    fn render_layout(&mut self, ui: &mut Ui) {
        ui.columns(2, |cols| {
            // الشريط الجانبي
            self.render_sidebar(&mut cols[0]);
            
            // المحتوى الرئيسي
            self.render_content(&mut cols[1]);
        });
    }
    
    fn render_sidebar(&self, ui: &mut Ui) {
        ui.add_space(10.0);
        
        ScrollArea::vertical().show(ui, |ui| {
            let pages = [
                Page::Dashboard,
                Page::Scanner,
                Page::Diagnostics,
                Page::Tools,
                Page::Advanced,
            ];
            
            for page in pages {
                let is_selected = self.current_page == page;
                
                let button = egui::Button::new(
                    egui::RichText::new(format!("{} {}", page.icon(), page.name()))
                        .size(15.0)
                )
                .min_size(egui::vec2(200.0, 45.0))
                .fill(if is_selected {
                    Color32::from_rgb(70, 130, 180)
                } else {
                    Color32::TRANSPARENT
                });
                
                if ui.add(button).clicked() {
                    // سيتم التعامل مع النقر في update التالي
                }
                ui.add_space(5.0);
            }
            
            ui.separator();
            
            // سجل الأحداث
            ui.collapsing("📋 سجل الأحداث", |ui| {
                ScrollArea::vertical()
                    .max_height(250.0)
                    .show(ui, |ui| {
                        for log in &self.log_messages {
                            ui.label(
                                egui::RichText::new(log)
                                    .size(11.0)
                                    .color(Color32::DARK_GRAY)
                            );
                        }
                    });
            });
        });
    }
    
    fn render_content(&mut self, ui: &mut Ui) {
        ui.add_space(10.0);
        
        match self.current_page {
            Page::Dashboard => DashboardPage::render(self, ui),
            Page::Scanner => ScannerPage::render(self, ui),
            Page::Diagnostics => DiagnosticsPage::render(self, ui),
            Page::Tools => ToolsPage::render(self, ui),
            Page::Advanced => AdvancedPage::render(self, ui),
        }
    }
    
    // Getters and setters for pages
    pub fn wifi_scanner_mut(&mut self) -> &mut WifiScanner {
        &mut self.wifi_scanner
    }
    
    pub fn wifi_scanner(&self) -> &WifiScanner {
        &self.wifi_scanner
    }
    
    pub fn diagnostics_mut(&mut self) -> &mut DiagnosticsEngine {
        &mut self.diagnostics
    }
    
    pub fn set_loading(&mut self, loading: bool) {
        self.loading = loading;
    }
    
    pub fn set_status(&mut self, status: impl Into<String>) {
        self.status_message = status.into();
    }
    
    pub fn add_log_message(&mut self, msg: impl Into<String>) {
        self.add_log(msg);
    }
}
