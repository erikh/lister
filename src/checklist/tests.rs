#![cfg(test)]
use super::*;
use anyhow::Result;

#[test]
fn test_to_from_string() -> Result<()> {
    let table = vec![
        (
            r#"
# This is a title
- item 1
  - item 2
  - item 3
    - item 4
  - item 5
- item 6
        "#,
            Checklist {
                title: "This is a title".to_string(),
                items: vec![
                    ChecklistItem::Item(
                        "item 1".to_string(),
                        vec![
                            ChecklistItem::Item("item 2".to_string(), Vec::new()),
                            ChecklistItem::Item(
                                "item 3".to_string(),
                                vec![ChecklistItem::Item("item 4".to_string(), Vec::new())],
                            ),
                            ChecklistItem::Item("item 5".to_string(), Vec::new()),
                        ],
                    ),
                    ChecklistItem::Item("item 6".to_string(), Vec::new()),
                ],
            },
        ),
        (
            r#"
# This is a title
- item 1
- item 2
- item 3
  - item 4
  - item 5
    - item 6
        "#,
            Checklist {
                title: "This is a title".to_string(),
                items: vec![
                    ChecklistItem::Item("item 1".to_string(), Vec::new()),
                    ChecklistItem::Item("item 2".to_string(), Vec::new()),
                    ChecklistItem::Item(
                        "item 3".to_string(),
                        vec![
                            ChecklistItem::Item("item 4".to_string(), Vec::new()),
                            ChecklistItem::Item(
                                "item 5".to_string(),
                                vec![ChecklistItem::Item("item 6".to_string(), Vec::new())],
                            ),
                        ],
                    ),
                ],
            },
        ),
        (
            r#"
# This is a title with checkboxes
- [ ] item 1
- item 2
- [x] item 3
  - item 4
  - [ ] item 5
    - item 6
        "#,
            Checklist {
                title: "This is a title with checkboxes".to_string(),
                items: vec![
                    ChecklistItem::Task(false, "item 1".to_string(), Vec::new()),
                    ChecklistItem::Item("item 2".to_string(), Vec::new()),
                    ChecklistItem::Task(
                        true,
                        "item 3".to_string(),
                        vec![
                            ChecklistItem::Item("item 4".to_string(), Vec::new()),
                            ChecklistItem::Task(
                                false,
                                "item 5".to_string(),
                                vec![ChecklistItem::Item("item 6".to_string(), Vec::new())],
                            ),
                        ],
                    ),
                ],
            },
        ),
    ];

    for (document, result) in table {
        let checklist: Checklist = document.into();
        assert_eq!(checklist, result);
        assert_eq!(result.to_string().trim(), document.trim());
    }

    Ok(())
}
