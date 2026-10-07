//! Decoupled NLE command registry.

pub struct Command {
    pub id: &'static str,
    pub label: &'static str,
    pub shortcut: Option<&'static str>,
}

pub const NLE_COMMANDS: &[Command] = &[
    Command { id: "clip.razor", label: "Split at Playhead", shortcut: Some("Cmd+K") },
    Command { id: "clip.ripple_delete", label: "Ripple Delete", shortcut: Some("Shift+Backspace") },
    Command { id: "seq.mark_in", label: "Mark In", shortcut: Some("I") },
    Command { id: "seq.mark_out", label: "Mark Out", shortcut: Some("O") },
    Command { id: "seq.snap_toggle", label: "Toggle Snapping", shortcut: Some("S") },
];
