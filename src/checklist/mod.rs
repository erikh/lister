mod parser;
#[cfg(test)]
mod tests;
mod tree;
use anyhow::{anyhow, Result};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Path(Vec<usize>);

impl Deref for Path {
    type Target = Vec<usize>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Path {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Path {
    fn overlap(&self, other: &Self) -> usize {
        let mut total = 0;
        for (x, item) in self.0.iter().enumerate() {
            if other[x] == *item {
                total += 1;
            }
        }
        total
    }
}

pub type ChecklistItems = Vec<ChecklistItem>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Checklist {
    pub title: String,
    pub items: ChecklistItems,
}

impl Checklist {
    pub fn find_path_mut(&mut self, path: Path) -> Option<(&mut ChecklistItems, usize)> {
        if path.is_empty() {
            return None;
        }

        let mut items = &mut self.items;
        for offset in &path[0..path.len() - 1] {
            if let Some(item) = items.get_mut(*offset) {
                items = item.items_mut();
            } else {
                return None;
            }
        }

        Some((items, path[path.len() - 1]))
    }

    pub fn find_path(&self, path: Path) -> Option<(ChecklistItems, usize)> {
        if path.is_empty() {
            return None;
        }

        let mut items = self.items.clone();
        for offset in &path[0..path.len() - 1] {
            if let Some(item) = items.get(*offset) {
                items = item.items().clone();
            } else {
                return None;
            }
        }

        Some((items, path[path.len() - 1]))
    }

    pub fn append(&mut self, path: Path, new_item: ChecklistItem) -> Result<()> {
        if path.is_empty() {
            self.items.push(new_item);
            return Ok(());
        }

        if let Some((items, offset)) = self.find_path_mut(path) {
            items[offset].items_mut().push(new_item);
            return Ok(());
        }

        Err(anyhow!("Could not locate path"))
    }

    pub fn path(&self, path: Path) -> Option<ChecklistItem> {
        if let Some((items, offset)) = self.find_path(path) {
            return Some(items[offset].clone());
        }

        None
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct ChecklistItem {
    title: String,
    position: Path,
    checked: Option<bool>,
    open: bool,
    items: ChecklistItems,
}

impl std::fmt::Display for ChecklistItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "{}{}\n",
            if let Some(checked) = self.checked {
                "[".to_owned() + if checked { "x" } else { " " } + "] "
            } else {
                "".to_owned()
            },
            self.title
        ))
    }
}

impl ChecklistItem {
    pub fn title(&self) -> String {
        self.title.clone()
    }
    pub fn items(&self) -> &ChecklistItems {
        &self.items
    }

    pub fn items_mut(&mut self) -> &mut ChecklistItems {
        &mut self.items
    }
}
