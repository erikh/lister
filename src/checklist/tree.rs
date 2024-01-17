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
            let mut cur = cur[0..cur.len() - x].to_vec();
            cur.push(pos);

            while first_pos != pos && pos > 0 && self.checklist.path(cur.clone()).unwrap().open {
                cur.pop();
                pos -= 1;
                cur.push(pos);
                total += 1;
            }

            if first_pos != pos {
                total += 1;
            }
        }

        total
    }

    pub fn current(&self) -> Path {
        self.current.clone()
    }

    pub fn set_current(&mut self, path: Path) {
        self.current = path
    }

    pub fn first_visible(&self) -> Path {
        self.first_visible.clone()
    }

    pub fn set_first_visible(&mut self, path: Path) {
        self.first_visible = path
    }

    fn move_count(&mut self, mut count: usize, direction: bool) -> bool {
        let cur = self.current();

        for (x, pos) in cur.iter().rev().enumerate() {
            let mut pos = pos.clone();
            loop {
                if pos == 0 && !direction {
                    // would be -1 if not for unsigned
                    break;
                }

                let next = if direction { pos + 1 } else { pos - 1 };

                // since we're counting in reverse, we subtract the enumeration index from the
                // length to get the right index to slice through.
                let mut cur = cur[0..cur.len() - x].to_vec();

                // add our next value to the list
                cur.push(next);

                // if we have an item here, decrease the count left to move.
                if let Some(item) = self.checklist.path(cur.clone()) {
                    if item.open {
                        count -= 1;
                        // If the count left to move is 0, set it to current and return that we
                        // moved successfully.
                        if count == 0 {
                            self.current = cur;
                            return true;
                        }
                    }
                } else {
                    break;
                }

                pos = next;
            }
        }

        // if we escaped the loop, we could not make enough moves. Current state is left
        // unmodified.
        false
    }

    pub fn move_up(&mut self, count: usize) -> bool {
        self.move_count(count, true)
    }

    pub fn move_down(&mut self, count: usize) -> bool {
        self.move_count(count, false)
    }
}
