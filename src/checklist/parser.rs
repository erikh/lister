use super::*;

lazy_static::lazy_static! {
    static ref TITLE_REGEX: regex::Regex = regex::Regex::new(r#"\s*#\s*([^\r\n]+)"#).unwrap();
    static ref LIST_REGEX: regex::Regex = regex::Regex::new(r#"([ \t]*)[-*]\s*(?:\[(.?)\])?\s*(.+?)(\s*\z|\s*\n\s*-)"#).unwrap();
}

impl std::fmt::Display for Checklist {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("# {}\n", self.title))?;

        let mut path = Path::default();
        let mut count = 0;
        let mut outer_items = Some(self.items.clone());

        loop {
            if let Some(items) = &outer_items {
                if let Some(item) = items.get(count) {
                    let mut indent = String::new();
                    for _ in 0..path.len() {
                        indent += "  "
                    }

                    f.write_str(&format!("{}- {}", indent, item))?;
                    let i = item.items();

                    if !i.is_empty() {
                        path.push(count);
                        count = 0;
                        outer_items = Some(i.clone());
                    } else {
                        count += 1;
                    }
                } else {
                    if let Some(c) = path.pop() {
                        count = c + 1;
                        outer_items = if path.is_empty() {
                            Some(self.items.clone())
                        } else {
                            Some(self.path(path.clone()).unwrap().items().clone())
                        };
                    } else {
                        outer_items = None;
                    }
                }
            } else {
                break;
            }
        }
        Ok(())
    }
}

impl From<String> for Checklist {
    fn from(value: String) -> Self {
        value.as_str().into()
    }
}

impl From<&str> for Checklist {
    fn from(value: &str) -> Self {
        let items = Vec::new();
        let mut title = String::new();
        let mut pos = 0;
        let mut locations = TITLE_REGEX.capture_locations();
        if let Some(m) = TITLE_REGEX.captures_read(&mut locations, value) {
            let loc = locations.get(1).unwrap();
            title = value[loc.0..loc.1].to_string();
            pos = m.end();
        }

        let mut obj = Self { title, items };

        let mut locations = LIST_REGEX.capture_locations();

        let mut space_list = Vec::new();
        let mut path = Path::default();
        let mut depth: Option<usize> = None;
        let mut count = 0;

        while let Some(_) = LIST_REGEX.captures_read_at(&mut locations, value, pos) {
            let spaces = locations.get(1).unwrap();
            let new_depth = spaces.1 - spaces.0;

            if let Some(tmp) = depth {
                if tmp > new_depth {
                    let mut spaces = space_list.pop();

                    loop {
                        count = path.pop().unwrap();
                        count += 1;

                        if spaces.is_none() {
                            break;
                        }

                        if tmp - spaces.unwrap() <= new_depth {
                            break;
                        }

                        if space_list.len() == 0 {
                            break;
                        }

                        spaces = Some(spaces.unwrap() + space_list.pop().unwrap());
                    }
                } else if tmp < new_depth {
                    space_list.push(new_depth - tmp);
                    path.push(count);
                    count = 0;
                } else {
                    count += 1;
                }
            }

            let checked = if let Some(checkbox) = locations.get(2) {
                if &value[checkbox.0..checkbox.1] == "x" {
                    Some(true)
                } else {
                    Some(false)
                }
            } else {
                None
            };

            let loc = locations.get(3).unwrap();
            let title = value[loc.0..loc.1].to_string();

            let item = if let Some(checked) = checked {
                ChecklistItem::Task(checked, title, Vec::new())
            } else {
                ChecklistItem::Item(title, Vec::new())
            };

            obj.append(path.clone(), item.clone()).unwrap();

            depth = Some(new_depth);

            pos = loc.1;
        }

        obj
    }
}
