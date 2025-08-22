use cursive::views::TextView;
use cursive::views::Dialog;
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

    siv.add_layer(
        Dialog::text("Have you used ICQ before?")  
        .title("retro test")
        .button("yes", yes)
        .button("no", no)
    );

    siv.run();
}

fn yes(s: &mut Cursive) {
    s.pop_layer();
    s.add_layer(TextView::new("Sweet!"));
}

fn no(s: &mut Cursive) {
    s.pop_layer();
    s.add_layer(TextView::new("Not Sweet!"));
}
