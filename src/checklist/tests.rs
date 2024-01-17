#![cfg(test)]
use super::*;
use anyhow::Result;

#[test]
fn test_to_from_string() -> Result<()> {
    let table = vec![
        (
            r#"
# This is a document with a lot of nesting
- item 1
  - item 2
  - item 3
    - item 4
  - item 5
  - item 6
    - item 7
- item 8
  - item 9
  - item 10
- item 11
        "#,
            Checklist {
                title: "This is a document with a lot of nesting".to_string(),
                items: vec![
                    ChecklistItem {
                        title: "item 1".to_string(),
                        items: vec![
                            ChecklistItem {
                                position: vec![0],
                                title: "item 2".to_string(),
                                ..Default::default()
                            },
                            ChecklistItem {
                                position: vec![0],
                                title: "item 3".to_string(),
                                items: vec![ChecklistItem {
                                    position: vec![0, 1],
                                    title: "item 4".to_string(),
                                    ..Default::default()
                                }],
                                ..Default::default()
                            },
                            ChecklistItem {
                                position: vec![0],
                                title: "item 5".to_string(),
                                ..Default::default()
                            },
                            ChecklistItem {
                                position: vec![0],
                                title: "item 6".to_string(),
                                items: vec![ChecklistItem {
                                    position: vec![0, 3],
                                    title: "item 7".to_string(),
                                    ..Default::default()
                                }],
                                ..Default::default()
                            },
                        ],
                        ..Default::default()
                    },
                    ChecklistItem {
                        title: "item 8".to_string(),
                        items: vec![
                            ChecklistItem {
                                position: vec![1],
                                title: "item 9".to_string(),
                                ..Default::default()
                            },
                            ChecklistItem {
                                position: vec![1],
                                title: "item 10".to_string(),
                                ..Default::default()
                            },
                        ],
                        ..Default::default()
                    },
                    ChecklistItem {
                        title: "item 11".to_string(),
                        ..Default::default()
                    },
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
                    ChecklistItem {
                        title: "item 1".to_string(),
                        ..Default::default()
                    },
                    ChecklistItem {
                        title: "item 2".to_string(),
                        ..Default::default()
                    },
                    ChecklistItem {
                        title: "item 3".to_string(),
                        items: vec![
                            ChecklistItem {
                                position: vec![2],
                                title: "item 4".to_string(),
                                ..Default::default()
                            },
                            ChecklistItem {
                                position: vec![2],
                                title: "item 5".to_string(),
                                items: vec![ChecklistItem {
                                    position: vec![2, 1],
                                    title: "item 6".to_string(),
                                    ..Default::default()
                                }],
                                ..Default::default()
                            },
                        ],
                        ..Default::default()
                    },
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
                    ChecklistItem {
                        checked: Some(false),
                        title: "item 1".to_string(),
                        ..Default::default()
                    },
                    ChecklistItem {
                        title: "item 2".to_string(),
                        ..Default::default()
                    },
                    ChecklistItem {
                        checked: Some(true),
                        title: "item 3".to_string(),
                        items: vec![
                            ChecklistItem {
                                position: vec![2],
                                title: "item 4".to_string(),
                                ..Default::default()
                            },
                            ChecklistItem {
                                position: vec![2],
                                checked: Some(false),
                                title: "item 5".to_string(),
                                items: vec![ChecklistItem {
                                    position: vec![2, 1],
                                    title: "item 6".to_string(),
                                    ..Default::default()
                                }],
                                ..Default::default()
                            },
                        ],
                        ..Default::default()
                    },
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
