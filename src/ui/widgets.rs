use egui::{Color32, Response, RichText, Ui, Widget};

pub struct StatusIndicator {
    pub status: bool,
    pub text: String,
}

impl Widget for StatusIndicator {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.horizontal(|ui| {
            let color = if self.status {
                Color32::GREEN
            } else {
                Color32::RED
            };
            
            ui.label(
                RichText::new("●")
                    .size(12.0)
                    .color(color)
            );
            
            ui.label(&self.text);
        })
        .response
    }
}

pub struct SignalBar {
    pub strength: i32,
}

impl Widget for SignalBar {
    fn ui(self, ui: &mut Ui) -> Response {
        let percentage = ((self.strength + 100) * 2).clamp(0, 100) as f32 / 100.0;
        let bars = (percentage * 4.0).round() as usize;
        
        ui.horizontal(|ui| {
            for i in 0..4 {
                let color = if i < bars {
                    match bars {
                        1 => Color32::RED,
                        2 => Color32::YELLOW,
                        _ => Color32::GREEN,
                    }
                } else {
                    Color32::DARK_GRAY
                };
                
                ui.label(
                    RichText::new("▮")
                        .size(16.0)
                        .color(color)
                );
            }
        })
        .response
    }
}

pub fn info_card<R>(ui: &mut Ui, title: &str, add_contents: impl FnOnce(&mut Ui) -> R) -> R {
    ui.group(|ui| {
        ui.label(RichText::new(title).strong().size(16.0));
        ui.separator();
        add_contents(ui)
    })
    .inner
}
