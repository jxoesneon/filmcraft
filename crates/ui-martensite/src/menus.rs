//! Accessible menu structure.

pub struct MenuCategory {
    pub name: &'static str,
    pub commands: &'static [&'static str],
}

pub const NLE_MENUS: &[MenuCategory] = &[
    MenuCategory { name: "File", commands: &["file.new", "file.open", "file.export"] },
    MenuCategory { name: "Edit", commands: &["edit.undo", "edit.redo", "clip.ripple_delete"] },
    MenuCategory { name: "Sequence", commands: &["seq.mark_in", "seq.mark_out", "seq.snap_toggle"] },
    MenuCategory { name: "Clip", commands: &["clip.razor"] },
];
