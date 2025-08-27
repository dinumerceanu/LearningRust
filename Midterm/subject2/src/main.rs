use core::time;
use std::path::Path;
use std::{os::windows::thread, path::PathBuf};
use std::fs;
use std::io::{self, Write};
use clap::builder::Str;
use clap::{Parser, Subcommand};
use cursive::view::Resizable;
use cursive::{align::HAlign, views::{Dialog, LinearLayout, SelectView, TextView}, Cursive, CursiveExt};

#[derive(Parser, Debug)]
#[command(version, about = "Changed about message")]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Clone, Debug, Subcommand)]
enum Commands {
    /// The browse command
    Browse {
        /// The path for browsing
        path: PathBuf,

        height: usize,

        width: usize,

        #[arg(long)]
        title: Option<String>,
    },
    /// The select command
    Select {
        height: usize,

        width: usize,

        #[arg(long)]
        title: Option<String>,

        #[arg(num_args = 1.., required = true)]
        items: Vec<String>
    },
}

fn ex4() {
    let mut siv = Cursive::new();

    let dialog = Dialog::around(
      TextView::new("This is a text")  
    )
    .title("This is a title");

    siv.add_layer(dialog);

    siv.run();
}

fn ex5(s: &mut Cursive, path: PathBuf) {
    let mut entries_list= SelectView::new().h_align(HAlign::Center);
    entries_list.add_item("..", "..".to_string());

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_name = entry.file_name().into_string().unwrap();
                entries_list.add_item(file_name.clone(), file_name);
            }
        }
    }

    let dialog = Dialog::around(entries_list).title("File browser");

    s.add_layer(dialog);
}

fn compare(a: &String, b: &String) -> std::cmp::Ordering {
    let path_a = a.parse::<PathBuf>().unwrap();
    let path_b = b.parse::<PathBuf>().unwrap();

    if path_a.is_dir() && path_b.is_file() {
        return std::cmp::Ordering::Less;
    } else if path_a.is_file() && path_b.is_dir() {
        return std::cmp::Ordering::Greater;
    } else {
        std::cmp::Ordering::Equal
    }
}

fn ex6(s: &mut Cursive, path: PathBuf) {
    let mut entries_list= SelectView::new().h_align(HAlign::Center);
    
    let mut entries_vec: Vec<String> = Vec::new();
    entries_vec.push("..".to_string());

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_name = entry.file_name().into_string().unwrap();
                entries_vec.push(file_name);
            }
        }
    }

    entries_vec.sort_by(compare);
    for entry in entries_vec {
        entries_list.add_item(entry.clone(), entry);
    }

    let dialog = Dialog::around(entries_list).title("File browser");

    s.add_layer(dialog);
}

fn ex8(s: &mut Cursive, path: PathBuf, height: usize, width: usize, title: Option<String>) {
    let mut entries_list= SelectView::new().h_align(HAlign::Center);
    
    let mut entries_vec: Vec<String> = Vec::new();
    entries_vec.push("..".to_string());

    if let Ok(entries) = fs::read_dir(path.clone()) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_name = entry.file_name().into_string().unwrap();
                entries_vec.push(file_name);
            }
        }
    }

    entries_vec.sort_by(compare);
    for entry in entries_vec {
        entries_list.add_item(entry.clone(), entry);
    }

    let title_clone = title.clone();

    let current_path = path;
    let on_submit = move |s: &mut Cursive, selected_item: &String| {
        let mut new_path = current_path.clone();
        new_path.push(selected_item);
        
        let new_path = new_path.canonicalize().unwrap_or(new_path);

        if new_path.is_dir() {
            s.pop_layer();
            ex8(s, new_path, height, width, title.clone());
        } else {
            s.set_user_data(new_path.to_string_lossy().to_string());
            s.quit();
        }
    };

    entries_list.set_on_submit(on_submit);

    let mut dialog = Dialog::around(entries_list).title("File browser");

    if let Some(t) = title_clone {
        dialog.set_title(t);
    }

    s.pop_layer();
    s.add_layer(dialog.fixed_size((width, height)));
}

fn select_command(s: &mut Cursive, height: usize, width: usize, title: Option<String>, items: Vec<String>) {
    let mut select = SelectView::new().h_align(HAlign::Center);
    for item in items {
        select.add_item(item.clone(), item);
    }

    select.set_on_submit(|s: &mut Cursive, selected_item: &String| {
        s.set_user_data(selected_item.clone());
        s.quit();
    });

    let mut dialog = Dialog::around(select);

    if let Some(t) = title {
        dialog.set_title(t);
        dialog.set_title_position(HAlign::Left);
    }

    s.add_layer(dialog.fixed_size((height, width)));
}

fn main() {
    let args_res = Args::try_parse();
    let mut siv = Cursive::new();

    match args_res {
        Ok(args) => {
            match args.cmd {
                Commands::Browse { path, height, width, title } => {
                    ex8(&mut siv, path, height, width, title);
                },
                Commands::Select { height, width, title, items } => {
                    select_command(&mut siv, height, width, title, items);
                },
            }

            siv.run();

            eprintln!("{}", siv.user_data::<String>().unwrap().clone());
        },
        Err(e) => {
            eprintln!("Error at parsing args: {}", e);
        }
    }
}
