use crate::checklist::{Checklist, ChecklistItem, ChecklistItems, Path};
use cursive::{traits::*, views::NamedView, Cursive};
use cursive_tree_view::{Placement, TreeView};

pub type ChecklistTree = TreeView<ChecklistItem>;

pub struct Tree {
    current: Path,
    checklist: Checklist,
}

fn collapse_func(siv: &mut Cursive, row: usize, is_collapsed: bool, children: usize) {
    if !is_collapsed && children == 0 {
        siv.call_on_name("tree", move |tree: &mut ChecklistTree| {
            tree_insert(
                tree,
                tree.borrow_item(row).unwrap().items().to_vec(),
                row,
                Placement::LastChild,
            );
        });
    }
}

fn tree_insert(
    tv: &mut ChecklistTree,
    items: ChecklistItems,
    parent: usize,
    placement: Placement,
) -> usize {
    for item in items {
        if item.items().is_empty() {
            tv.insert_item(item, placement, parent);
        } else {
            tv.insert_container_item(item.clone(), placement, parent);
        }
    }

    parent
}

impl Tree {
    pub fn new(checklist: Checklist) -> Self {
        Tree {
            current: Path::default(),
            checklist,
        }
    }

    pub fn title(&self) -> String {
        self.checklist.title.clone()
    }

    pub fn next_path(&self) -> Path {
        let mut path = self.current_path();
        while let Some(idx) = path.pop() {
            if let Some(item) = self.checklist.path(path.clone()) {
                if item.items().len() - 1 > idx {
                    path.push(idx + 1);
                    return path;
                }
            }
        }

        return Path::default();
    }

    pub fn last_path(&self) -> Path {
        let mut path = self.current_path();
        while let Some(idx) = path.pop() {
            if let Some(item) = self.checklist.path(path.clone()) {
                if idx > 0 && !item.items().is_empty() {
                    path.push(idx - 1);
                    return path;
                }
            }
        }

        return Path::default();
    }

    pub fn move_item(&mut self, _from: Path, _to: Path) {}
    pub fn delete_item(&mut self, _path: Path) {}

    pub fn set_current(&mut self, path: Path) {
        self.current = path;
    }

    pub fn current_path(&self) -> Path {
        self.current.clone()
    }

    pub fn current_item(&self) -> ChecklistItem {
        self.checklist.path(self.current.clone()).unwrap()
    }

    pub fn as_treeview(&self) -> NamedView<ChecklistTree> {
        let mut tv = ChecklistTree::new();
        tree_insert(&mut tv, self.checklist.items.clone(), 0, Placement::After);
        tv.set_on_collapse(collapse_func);
        tv.with_name("tree")
    }
}
