mod parser;
#[cfg(test)]
mod tests;
use anyhow::{anyhow, Result};

pub type Path = Vec<usize>;
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ChecklistItem {
    Item(String, ChecklistItems),
    Task(bool, String, ChecklistItems),
}

impl std::fmt::Display for ChecklistItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChecklistItem::Task(complete, title, _) => f.write_str(&format!(
                "[{}] {}\n",
                if *complete { "x" } else { " " },
                title
            )),
            ChecklistItem::Item(title, _) => f.write_str(&format!("{}\n", title)),
        }
    }
}

impl ChecklistItem {
    pub fn title(&self) -> String {
        match self {
            ChecklistItem::Item(title, _) | ChecklistItem::Task(_, title, _) => title.clone(),
        }
    }
    pub fn items(&self) -> &ChecklistItems {
        match self {
            ChecklistItem::Item(_, new_items) | ChecklistItem::Task(_, _, new_items) => new_items,
        }
    }

    pub fn items_mut(&mut self) -> &mut ChecklistItems {
        match self {
            ChecklistItem::Item(_, new_items) | ChecklistItem::Task(_, _, new_items) => new_items,
        }
    }
}
