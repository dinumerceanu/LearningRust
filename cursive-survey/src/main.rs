use cursive::view::{Nameable, Resizable};
use cursive::views::{Checkbox, Dialog, EditView, LinearLayout, RadioGroup, StackView, TextView};
use cursive::{Cursive, CursiveExt, With};

#[derive(Default)]
struct SurveyData {
    name: String,
    surname: String,
    gender: String,
    eye_color: String,
    answers: Vec<String>,
}

fn start_survey(s: &mut Cursive) {
    s.call_on_name("stack", |stack: &mut StackView| {
        stack.add_layer(
             Dialog::around(TextView::new("Are you ready to start the survey?"))
            .title("Survey Boss")
            .button("Yes!", |s| user_personal_data(s))
            .button("No...", |s| s.quit())
        );
    });
}

fn user_personal_data(s: &mut Cursive) {
    s.call_on_name("stack", |stack: &mut StackView| {
        let mut eyes_color_group: RadioGroup<&str> = RadioGroup::new();

        let form = LinearLayout::vertical()
            .child(TextView::new("Name"))
            .child(EditView::new().with_name("name"))
            .child(TextView::new("Surname"))
            .child(EditView::new().with_name("surname"))
            .child(TextView::new("Gender"))
            .child(
                LinearLayout::horizontal()
                    .child(Checkbox::new().with_name("male"))
                    .child(TextView::new(" Male "))
                    .child(Checkbox::new().with_name("female"))
                    .child(TextView::new(" Female "))
            )
            .child(TextView::new("Eye color"))
            .child(LinearLayout::vertical()
                .child(eyes_color_group.button("blue", "blue"))
                .child(eyes_color_group.button("green", "green"))
                .child(eyes_color_group.button("brown", "brown"))
            );

        stack.pop_layer();
        stack.add_layer(
            Dialog::around(form)
                .title("Enter your data")
                .button("Next", move |s| {
                    let is_male = s.call_on_name("male", |v: &mut Checkbox| v.is_checked()).unwrap();
                    let is_female = s.call_on_name("female", |v: &mut Checkbox| v.is_checked()).unwrap();

                    if !(is_male || is_female) {
                        s.add_layer(Dialog::info("Please select an option (Male/Female)."));
                    } else if is_male && is_female {
                        s.add_layer(Dialog::info("Please choose only one option (Male/Female)."));
                    } else {
                        let name = s.call_on_name("name", |v: &mut EditView| v.get_content()).unwrap();
                        let surname = s.call_on_name("surname", |v: &mut EditView| v.get_content()).unwrap();
                        let gender = if is_male { "male".to_string() } else { "female".to_string() };
                        let eye_color = eyes_color_group.selection();

                        s.with_user_data(|data: &mut SurveyData| {
                            data.name = name.to_string();
                            data.surname = surname.to_string();
                            data.gender = gender;
                            data.eye_color = eye_color.to_string();
                        });

                        second_page(s);
                    }
                })
                .button("Back", |s| {
                    s.call_on_name("stack", |stack: &mut StackView| stack.pop_layer());
                })
        );
    });
}

fn second_page(s: &mut Cursive) {
    s.call_on_name("stack", |stack: &mut StackView| {
        stack.pop_layer();
        stack.add_layer(
            Dialog::around(
                EditView::new().with_name("bench")
            )
            .title("How much do you bench press bro(KG)?")
            .button("Next", |s: &mut Cursive| {
                let answear = s.call_on_name("bench", |v: &mut EditView| {
                    v.get_content()
                });

                if let Some(arc_str) = answear {
                    let str: &str = &arc_str;
                    match str.parse::<usize>() {
                        Ok(num) => {
                            s.with_user_data(|data: &mut SurveyData| {
                                let res = format!("Bench press: {}", num);
                                data.answers.push(res)
                            });
                            final_page(s);
                        },
                        Err(_) => {
                            s.add_layer(Dialog::info("Please enter a valid number"));
                        }
                    }
                } else {
                    s.add_layer(Dialog::info("Please enter a valid number"));
                }
            })
            .button("Back", |s| {
                s.call_on_name("stack", |stack: &mut StackView| stack.pop_layer());
            })
        );
    });
}

fn final_page(s: &mut Cursive) {
    let stats = s
        .with_user_data(|data: &mut SurveyData| {
            format!(
                "Your name is {} {}, you have {} eyes, and you are a strong {}",
                data.name, data.surname, data.eye_color, data.gender
            )
        })
        .unwrap();

    s.call_on_name("stack", |stack: &mut StackView| {
        stack.pop_layer();
        stack.add_layer(TextView::new(stats));
    });
}

fn main() {
    let mut siv = Cursive::new();
    siv.set_user_data(SurveyData::default());

    siv.add_layer(StackView::new().with_name("stack"));

    start_survey(&mut siv);

    siv.run();
}
