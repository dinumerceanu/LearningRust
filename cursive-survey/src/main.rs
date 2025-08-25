use cursive::view::{Nameable, Resizable};
use cursive::views::{Checkbox, Dialog, EditView, LinearLayout, RadioGroup, TextView};
use cursive::{Cursive, CursiveExt};

fn first_page(s: &mut Cursive) {
    s.add_layer(
        Dialog::around(TextView::new("Are you ready to start the survey?"))
        .title("Survey Boss")
        .button("Yes!", start_survey)
        .button("No...", |s| s.quit())
    );
}

fn start_survey(s: &mut Cursive) {
    s.pop_layer();

    let mut eyes_color_group: RadioGroup<&str> = RadioGroup::new();

    let form = LinearLayout::vertical()
        .child(Dialog::around(EditView::new().with_name("name")).title("name"))
        .child(Dialog::around(EditView::new().with_name("surname")).title("surname"))
        .child(LinearLayout::vertical()
            .child(
                LinearLayout::horizontal()
                    .child(Checkbox::new().with_name("male"))
                    .child(TextView::new("male"))
            )
            .child(LinearLayout::horizontal()
                    .child(Checkbox::new().with_name("female"))
                    .child(TextView::new("female"))
                )
        )
        .child(
            LinearLayout::vertical()
                .child(eyes_color_group.button("blue", "blue"))
                .child(eyes_color_group.button("green", "green"))
                .child(eyes_color_group.button("brown", "brown"))
        );

    s.add_layer(
        Dialog::around(form)
        .title("Enter your data")
        .button("Send", move |s| {
            let is_male = s.call_on_name("male", |v: &mut Checkbox| {
                    v.is_checked()
                }).unwrap();

            let is_female = s.call_on_name("female", |v: &mut Checkbox| {
                    v.is_checked()
                }).unwrap();

            if !(is_male || is_female) {
                s.add_layer(Dialog::info("Please select an option (Male/Female)."));
            } else if is_male && is_female {
                s.add_layer(Dialog::info("Please choose only one option (Male/Female)."));
            } else {
                let name = s.call_on_name("name", |v: &mut EditView| {
                    v.get_content()
                }).unwrap();

                let surname = s.call_on_name("surname", |v: &mut EditView| {
                    v.get_content()
                }).unwrap();

                let gender = if is_male {
                    "male".to_string()
                } else {
                    "female".to_string()
                };
                let eye_color = eyes_color_group.selection();

                let res = format!("Hello, {} {}, you are a {} and you have {} eyes!", name, surname, gender, eye_color);

                s.pop_layer();
                s.add_layer(TextView::new(res));
            }
        })
    );
}



fn main() {
    let mut siv = Cursive::new();

    first_page(&mut siv);

    siv.run();
}
