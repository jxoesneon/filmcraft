//! Video editing theme tokens for FilmCraft.

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }
}

pub struct FilmTheme {
    pub surface_timeline: Color,
    pub surface_monitor: Color,
    pub track_video: Color,
    pub track_audio: Color,
    pub playhead_needle: Color,
    pub marker_in_out: Color,
}

impl FilmTheme {
    pub fn dark_nle() -> Self {
        Self {
            surface_timeline: Color::rgb(24, 26, 32),
            surface_monitor: Color::rgb(14, 16, 20),
            track_video: Color::rgb(45, 60, 85),
            track_audio: Color::rgb(40, 75, 60),
            playhead_needle: Color::rgb(0, 225, 255),
            marker_in_out: Color::rgb(255, 180, 0),
        }
    }
}
