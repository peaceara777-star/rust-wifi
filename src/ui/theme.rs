use egui::{Color32, Visuals};

#[derive(Clone, Copy, PartialEq)]
pub enum ThemeMode {
    Dark,
    Light,
}

pub struct AppTheme {
    mode: ThemeMode,
}

impl Default for AppTheme {
    fn default() -> Self {
        Self {
            mode: ThemeMode::Dark,
        }
    }
}

impl AppTheme {
    pub fn toggle(&mut self) {
        self.mode = match self.mode {
            ThemeMode::Dark => ThemeMode::Light,
            ThemeMode::Light => ThemeMode::Dark,
        };
    }
    
    pub fn get_visuals(&self) -> Visuals {
        match self.mode {
            ThemeMode::Dark => {
                let mut visuals = Visuals::dark();
                visuals.panel_fill = Color32::from_rgb(30, 30, 35);
                visuals.window_fill = Color32::from_rgb(40, 40, 45);
                visuals.selection.bg_fill = Color32::from_rgb(70, 130, 180);
                visuals.hyperlink_color = Color32::from_rgb(100, 180, 255);
                visuals
            }
            ThemeMode::Light => {
                let mut visuals = Visuals::light();
                visuals.panel_fill = Color32::from_rgb(245, 245, 245);
                visuals.window_fill = Color32::from_rgb(255, 255, 255);
                visuals.selection.bg_fill = Color32::from_rgb(70, 130, 180);
                visuals.hyperlink_color = Color32::from_rgb(0, 100, 200);
                visuals
            }
        }
    }
}
