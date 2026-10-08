//! End-to-end NLE integration tests.

use filmcraft_engine::Engine;
use filmcraft_ui_martensite::{
    FilmcraftApp, NleTool,
    widgets::meters::AudioMeterWidget,
    widgets::timeline::{Clip, TimelineWidget},
};

#[test]
fn test_nle_editing_workflow() {
    let mut app = FilmcraftApp::new(Engine::default());
    let mut timeline = TimelineWidget::new();

    timeline.clips.push(Clip { id: 1, start_frame: 0, duration_frames: 300, track_index: 0 });

    // 1. Razor Tool Workflow
    let razor = app.keyboard.on_key_down("c");
    assert_eq!(razor, Some(NleTool::Razor));
    app.active_tool = NleTool::Razor;

    // 2. Split Clip at frame 120
    assert!(timeline.split_clip(1, 120));
    assert_eq!(timeline.clips.len(), 2);
    assert_eq!(timeline.clips[0].duration_frames, 120);
    assert_eq!(timeline.clips[1].start_frame, 120);
    assert_eq!(timeline.clips[1].duration_frames, 180);

    // 3. Audio Meter Updates
    let mut meters = AudioMeterWidget::new();
    meters.update_levels(-12.0, -14.5);
    assert_eq!(meters.left_peak_db, -12.0);
    assert_eq!(meters.right_peak_db, -14.5);
}
