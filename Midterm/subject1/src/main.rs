use std::os::windows::process;

use clap::builder::Str;
use clap::{Parser, Subcommand};
use cursive::view::{Nameable, Resizable};
use cursive::{Cursive, CursiveExt};
use cursive::views::{Button, Dialog, EditView, LinearLayout, TextView};
  
#[derive(Parser, Debug)]
#[command(about = "Changed help message")]
struct Args {
    #[command(subcommand)]
    cmd: Commands,

    #[arg(long, global = true)]
    title: Option<String>,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Message {
        text_message: String,

        height: usize,

        width: usize,

        // #[arg(long)]
        // title: Option<String>
    },

    Input {
        text_message: String,

        height: usize,

        width: usize,

        // #[arg(long)]
        // title: Option<String>
    },

    #[command(name = "yes_no")]
    YesNo {
        question: String,

        height: usize,

        width: usize,

        // #[arg(long)]
        // title: Option<String>,

        #[arg(long)]
        yes: String,

        #[arg(long)]
        no: String,
    },
}

fn ex4(s: &mut Cursive) {
    s.pop_layer();
    s.add_layer(
        Dialog::around(
            TextView::new("Chenar")
        )
        .title("Titlu")
    );
}

fn ex5(s: &mut Cursive) {
    s.pop_layer();
    s.add_layer(
        Dialog::around(
            EditView::new().with_name("input")
        ).title("Please enter your input here:")
    );   
}

fn ex6(s: &mut Cursive) {
    s.pop_layer();
    s.add_layer(
        Dialog::around(
            TextView::new("Do you like apples?")
        ).title("Answear the following question:")
        .button("yes", yes)
        .button("no", no)
    );   
}

fn yes(s: &mut Cursive) {

}

fn no(s: &mut Cursive) {
    
}

fn main() {
    let args = Args::parse();
    let mut siv = Cursive::new();
    
    match args.cmd {
        Commands::Message { text_message, height, width } => {
            let mut dialog = Dialog::around(
                        TextView::new(text_message)
                    )
                    .button("ok", |s: &mut Cursive| s.quit());

            if let Some(t) = args.title {
                dialog.set_title(t);
                dialog.set_title_position(cursive::align::HAlign::Left);
            }
            let sized_dialog = dialog.fixed_size((height, width));
            siv.add_layer(sized_dialog);
        },
        Commands::Input { text_message, height, width } => {
            let layout = LinearLayout::vertical()
                .child(TextView::new(text_message))
                .child(EditView::new().with_name("input"));

            let mut dialog = Dialog::around(layout)
                .button("ok", |s: &mut Cursive| {
                    let input = s.call_on_name("input", |v: &mut EditView| {
                        v.get_content()
                    });
                    s.quit();
                    eprintln!("{}", &*input.unwrap());
                });
            
            if let Some(t) = args.title {
                dialog.set_title(t);
                dialog.set_title_position(cursive::align::HAlign::Left);
            }
            
            let sized_dialog = dialog.fixed_size((height, width));
            siv.add_layer(sized_dialog);
        },
        Commands::YesNo { question, height, width, yes, no} => {
            let mut dialog =
                Dialog::around(
                    TextView::new("Do you like apples?")
                )
                .button(yes, |s: &mut Cursive| {
                    s.set_user_data(0);
                    s.quit();
                })
                .button(no, |s: &mut Cursive| {
                    s.set_user_data(1);
                    s.quit();
                });
            if let Some(t) = args.title {
                dialog.set_title(t);
                dialog.set_title_position(cursive::align::HAlign::Left);
            }
            let sized_dialog = dialog.fixed_size((height, width));
            siv.add_layer(sized_dialog);
        },
        _ => {}
    }

    // let mut siv = Cursive::new();

    // ex4(&mut siv);

    // ex5(&mut siv);

    // ex6(&mut siv);

    siv.run();

    if let Some(exit_code) = siv.take_user_data::<i32>() {
        std::process::exit(exit_code);
    }
}
