//! NLE commands and keyboard shortcut dispatch.

use crate::NleTool;

pub struct NleKeyboardEngine {
    pub jkl_speed: i32, // -8x to +8x shuttle
}

impl NleKeyboardEngine {
    pub fn new() -> Self {
        Self { jkl_speed: 0 }
    }

    pub fn on_key_down(&mut self, key: &str) -> Option<NleTool> {
        match key {
            "v" | "V" => Some(NleTool::Selection),
            "a" | "A" => Some(NleTool::TrackSelectForward),
            "b" | "B" => Some(NleTool::RippleEdit),
            "n" | "N" => Some(NleTool::RollingEdit),
            "r" | "R" => Some(NleTool::RateStretch),
            "c" | "C" => Some(NleTool::Razor),
            "y" | "Y" => Some(NleTool::Slip),
            "u" | "U" => Some(NleTool::Slide),
            "h" | "H" => Some(NleTool::Hand),
            "z" | "Z" => Some(NleTool::Zoom),
            _ => None,
        }
    }

    pub fn handle_jkl(&mut self, key: &str) -> i32 {
        match key {
            "j" | "J" => {
                if self.jkl_speed > 0 { self.jkl_speed = -1; }
                else { self.jkl_speed = (self.jkl_speed * 2).clamp(-8, -1); }
            }
            "k" | "K" => { self.jkl_speed = 0; }
            "l" | "L" => {
                if self.jkl_speed < 0 { self.jkl_speed = 1; }
                else { self.jkl_speed = (self.jkl_speed * 2).max(1).clamp(1, 8); }
            }
            _ => {}
        }
        self.jkl_speed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nle_tool_shortcuts() {
        let mut k = NleKeyboardEngine::new();
        assert_eq!(k.on_key_down("c"), Some(NleTool::Razor));
        assert_eq!(k.on_key_down("v"), Some(NleTool::Selection));
        assert_eq!(k.on_key_down("b"), Some(NleTool::RippleEdit));
    }

    #[test]
    fn test_jkl_shuttle() {
        let mut k = NleKeyboardEngine::new();
        assert_eq!(k.handle_jkl("l"), 1);
        assert_eq!(k.handle_jkl("l"), 2);
        assert_eq!(k.handle_jkl("k"), 0);
        assert_eq!(k.handle_jkl("j"), -1);
    }
}
