//! Timeline and audio meter widgets.

pub mod timeline {
    #[derive(Clone, Debug, PartialEq)]
    pub struct Clip {
        pub id: u64,
        pub start_frame: u64,
        pub duration_frames: u64,
        pub track_index: usize,
    }

    pub struct TimelineWidget {
        pub clips: Vec<Clip>,
        pub selected_clip: Option<u64>,
    }

    impl TimelineWidget {
        pub fn new() -> Self {
            Self { clips: Vec::new(), selected_clip: None }
        }

        pub fn split_clip(&mut self, clip_id: u64, cut_frame: u64) -> bool {
            if let Some(pos) = self.clips.iter().position(|c| c.id == clip_id) {
                let clip = &self.clips[pos];
                if cut_frame > clip.start_frame && cut_frame < clip.start_frame + clip.duration_frames {
                    let first_dur = cut_frame - clip.start_frame;
                    let second_dur = clip.duration_frames - first_dur;
                    let track = clip.track_index;
                    let new_id = clip.id + 10000;
                    self.clips[pos].duration_frames = first_dur;
                    self.clips.push(Clip { id: new_id, start_frame: cut_frame, duration_frames: second_dur, track_index: track });
                    return true;
                }
            }
            false
        }
    }
}

pub mod meters {
    pub struct AudioMeterWidget {
        pub left_peak_db: f32,
        pub right_peak_db: f32,
    }

    impl AudioMeterWidget {
        pub fn new() -> Self {
            Self { left_peak_db: -60.0, right_peak_db: -60.0 }
        }

        pub fn update_levels(&mut self, l: f32, r: f32) {
            self.left_peak_db = l.clamp(-60.0, 6.0);
            self.right_peak_db = r.clamp(-60.0, 6.0);
        }
    }
}
