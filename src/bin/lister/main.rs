use anyhow::Result;
use clap::Parser;
use cursive::{
    self,
    theme::{BaseColor, BorderStyle, Color, Palette, PaletteColor, Theme},
    traits::*,
    views::{Dialog, TextContent, TextView},
};
use lister_cli::tree::*;
use std::{
    fs::read_to_string,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = include_str!("../../../README.md"))]
struct Cli {
    /// Name of file to interact with
    filename: std::path::PathBuf,
}

fn main() -> Result<()> {
    let help = Arc::new(AtomicBool::default());

    let cli = Cli::parse();
    let tree = Tree::new(read_to_string(cli.filename)?.into());

    let mut siv = cursive::default();

    let mut theme = Theme::default();
    theme.shadow = false;
    theme.borders = BorderStyle::Simple;
    let mut palette = Palette::terminal_default();
    palette[PaletteColor::Primary] = Color::Light(BaseColor::Black);
    palette[PaletteColor::Secondary] = Color::Light(BaseColor::Cyan);
    palette[PaletteColor::Tertiary] = Color::Dark(BaseColor::Cyan);
    palette[PaletteColor::TitlePrimary] = Color::Light(BaseColor::White);
    palette[PaletteColor::TitleSecondary] = Color::Dark(BaseColor::White);
    palette[PaletteColor::HighlightText] = Color::Dark(BaseColor::Cyan);
    theme.palette = palette;
    siv.set_theme(theme);

    siv.add_layer(
        Dialog::around(tree.as_treeview().scrollable())
            .title(tree.title())
            .full_screen()
            .with_name("main"),
    );

    let h = help.clone();

    siv.add_global_callback('?', move |s| {
        let res = h.load(Ordering::Relaxed);
        h.store(!res, Ordering::Relaxed);
        if res {
            s.pop_layer();
        } else {
            s.add_layer(
                Dialog::around(TextView::new_with_content(TextContent::new(
                    r#"
Lister takes markdown lists and turns them into navigable UI

Help:

? - This Help
<Down Arrow>, j - Move Down
<Up Arrow>, k - Move Up
<Enter>, <Space> - Expand or Contract a Sub-List
> - Expand all sub-trees
< - Collapse all sub-trees
q - Quit this Program

"#,
                )))
                .title("Help"),
            );
        }
    });

    siv.add_global_callback('>', |s| {
        let mut tree = s.find_name::<ChecklistTree>("tree").unwrap();
        for r in 0..tree.len() {
            tree.set_collapsed(r, false);
        }
    });

    siv.add_global_callback('<', |s| {
        let mut tree = s.find_name::<ChecklistTree>("tree").unwrap();
        for r in 0..tree.len() {
            tree.set_collapsed(r, true);
        }
    });

    siv.add_global_callback('j', |s| {
        s.on_event(cursive::event::Event::Key(cursive::event::Key::Down));
    });

    siv.add_global_callback('k', |s| {
        s.on_event(cursive::event::Event::Key(cursive::event::Key::Up));
    });

    siv.add_global_callback(' ', |s| {
        s.on_event(cursive::event::Event::Key(cursive::event::Key::Enter));
    });

    let h = help.clone();
    siv.add_global_callback('q', move |s| {
        if h.load(Ordering::Relaxed) {
            s.pop_layer();
            h.store(false, Ordering::Relaxed);
        } else {
            s.quit();
        }
    });

    siv.run();

    Ok(())
}
