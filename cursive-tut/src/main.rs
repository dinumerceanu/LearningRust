use cursive::view::Nameable;
use cursive::views::{Dialog, TextView, Checkbox, LinearLayout};
use cursive::{Cursive, CursiveExt};

fn main() {
    let mut siv = Cursive::new();

    // siv.add_layer(TextView::new("Hello World!\nPress q to quit."));

    // siv.add_global_callback('q', |s| s.quit());

    // siv.add_layer(Dialog::around(TextView::new("What the fuck is going on here?")));

    // siv.add_layer(Dialog::text("10 la restanta").title("NOTA"));

    // siv.add_layer(Dialog::text("Have you finished working or not yet?")
    // .title("Personal Organizer")
    // .button("Yes!", |s| s.quit())
    // .button("No!", |s| s.quit())
    // .button("Not sure...", |s| s.quit())
    // );

    // siv.add_layer(
    //     Dialog::text("Have you used ICQ before?")  
    //     .title("retro test")
    //     .button("yes", yes)
    //     .button("no", no)
    // );


    let survey = LinearLayout::vertical()
        .child(
            LinearLayout::horizontal()
                .child(Checkbox::new().with_name("rust"))
                .child(TextView::new("Rust")),
        )
        .child(
            LinearLayout::horizontal()
                .child(Checkbox::new().with_name("go"))
                .child(TextView::new("Go")),
        )
        .child(
            LinearLayout::horizontal()
                .child(Checkbox::new().with_name("python"))
                .child(TextView::new("Python")),
        )
        .child(
            LinearLayout::horizontal()
                .child(Checkbox::new().with_name("cpp"))
                .child(TextView::new("C++")),
        );

    siv.add_layer(
        Dialog::around(survey)
            .title("Ce limbaje îți plac?")
            .button("Trimite", |s| {
                let rust = s.call_on_name("rust", |v: &mut Checkbox| v.is_checked()).unwrap();
                let go = s.call_on_name("go", |v: &mut Checkbox| v.is_checked()).unwrap();
                let python = s.call_on_name("python", |v: &mut Checkbox| v.is_checked()).unwrap();
                let cpp = s.call_on_name("cpp", |v: &mut Checkbox| v.is_checked()).unwrap();

                let rez = format!(
                    "Rezultate:\nRust: {}\nGo: {}\nPython: {}\nC++: {}",
                    rust, go, python, cpp
                );

                s.pop_layer();
                s.add_layer(TextView::new(rez));
            })
            .button("Renunță", |s| s.quit()),
    );

    siv.run();
}

// fn yes(s: &mut Cursive) {
//     s.pop_layer();
//     s.add_layer(TextView::new("Sweet!"));
// }

// fn no(s: &mut Cursive) {
//     s.pop_layer();
//     s.add_layer(TextView::new("Not Sweet!"));
// }
