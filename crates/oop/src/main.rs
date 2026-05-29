use crate::{blog::*, blog2::*, gui::*};

pub mod blog;
pub mod blog2;
pub mod gui;

#[allow(dead_code)]
fn traitobject_test() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("Ok"),
            }),
        ],
    };

    screen.run();
}

#[allow(dead_code)]
fn state_pattern_test() {
    let mut post = Post::new();

    // Draft
    post.add_text("UwU. What's this?");
    assert_eq!("", post.content());

    post.add_text("\nFirst test.");
    println!("{}", post.content());

    // PendingReview
    post.request_review();
    assert_eq!("", post.content());

    post.add_text("\nSecond test.");
    println!("{}", post.content());

    // Published
    post.approve();
    post.approve();
    assert_eq!("UwU. What's this?\nFirst test.", post.content());
    println!("{}", post.content());
}

#[allow(dead_code)]
fn state_pattern_test2() {
    let mut post = Post2::new();

    post.add_text("^^");
    let post = post.request_review();

    let post = post.approve();

    let post = post.approve();

    println!("{}", post.content());
}

fn main() {
    state_pattern_test2();
}
