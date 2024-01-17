#![allow(dead_code)]
use super::*;

pub struct ChecklistTree {
    checklist: Checklist,
    current: Path,
    first_visible: Path,
}

impl ChecklistTree {
    fn position_from_top(&self) -> usize {
        self.current.overlap(&self.first_visible)
    }
}
