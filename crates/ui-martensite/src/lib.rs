//! Sovereign retained-mode NLE interface for FilmCraft built on the Martensite GUI engine.

pub mod command_reg;
pub mod menus;
pub mod shortcuts;
pub mod theme;
pub mod widgets;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NleTool {
    Selection,      // V
    TrackSelectForward, // A
    RippleEdit,     // B
    RollingEdit,    // N
    RateStretch,    // R
    Razor,          // C
    Slip,           // Y
    Slide,          // U
    Hand,           // H
    Zoom,           // Z
}

pub struct FilmcraftApp {
    pub theme: theme::FilmTheme,
    pub keyboard: shortcuts::NleKeyboardEngine,
    pub active_tool: NleTool,
    pub playhead_frame: u64,
    pub duration_frames: u64,
    pub is_playing: bool,
    pub in_point: Option<u64>,
    pub out_point: Option<u64>,
    pub snapping_enabled: bool,
}

impl FilmcraftApp {
    pub fn new() -> Self {
        Self {
            theme: theme::FilmTheme::dark_nle(),
            keyboard: shortcuts::NleKeyboardEngine::new(),
            active_tool: NleTool::Selection,
            playhead_frame: 0,
            duration_frames: 1800, // 60s at 30fps
            is_playing: false,
            in_point: None,
            out_point: None,
            snapping_enabled: true,
        }
    }

    pub fn toggle_play(&mut self) -> bool {
        self.is_playing = !self.is_playing;
        self.is_playing
    }

    pub fn seek_to(&mut self, frame: u64) {
        self.playhead_frame = frame.min(self.duration_frames);
    }

    pub fn set_in_point(&mut self) {
        self.in_point = Some(self.playhead_frame);
    }

    pub fn set_out_point(&mut self) {
        self.out_point = Some(self.playhead_frame);
    }

    pub fn clear_in_out(&mut self) {
        self.in_point = None;
        self.out_point = None;
    }

    pub fn toggle_snapping(&mut self) -> bool {
        self.snapping_enabled = !self.snapping_enabled;
        self.snapping_enabled
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_lifecycle() {
        let mut app = FilmcraftApp::new();
        assert_eq!(app.playhead_frame, 0);
        assert!(!app.is_playing);

        assert!(app.toggle_play());
        assert!(app.is_playing);

        app.seek_to(500);
        assert_eq!(app.playhead_frame, 500);

        app.set_in_point();
        assert_eq!(app.in_point, Some(500));

        app.seek_to(1200);
        app.set_out_point();
        assert_eq!(app.out_point, Some(1200));

        app.clear_in_out();
        assert_eq!(app.in_point, None);
        assert_eq!(app.out_point, None);
    }
}
