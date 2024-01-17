#![allow(dead_code)]
use super::*;

pub struct ChecklistTree {
    checklist: Checklist,
    current: Path,
    first_visible: Path,
}

impl ChecklistTree {
    fn position_from_top(&self) -> usize {
        let mut total = 0;

        let cur = self.current.clone();
        for (x, pos) in cur.iter().rev().enumerate() {
            let mut pos = pos.clone();
            let first_pos = self.first_visible[cur.len() - x];
            while first_pos != pos && pos > 0 {
                pos -= 1;
                total += 1;
            }
            if first_pos != pos {
                total += 1;
            }
        }

        total
    }
}
