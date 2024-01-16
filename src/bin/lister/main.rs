use anyhow::Result;
use clap::Parser;
use cursive::{
    self,
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
q - Quit this Program

"#,
                )))
                .title("Help"),
            );
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
