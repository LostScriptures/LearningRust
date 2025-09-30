#[allow(unused_imports)]
use crate::{
    closures::*, concurrency::*, generics::*, iterators::*, shapes::*, spointers::*, vecs::*,
};
use std::vec;

pub mod closures;
pub mod concurrency;
pub mod custombintree;
pub mod errors;
pub mod generics;
pub mod iterators;
pub mod shapes;
pub mod spointers;
pub mod vecs;

pub fn test() {
    println!("Wrong way around ^^");
}

#[allow(dead_code)]
fn error_test() {
    match errors::read_username() {
        Ok(name) => println!("Username: {name}"),
        Err(e) => panic!("An Error ocurred: {e:?}"),
    }
}

#[allow(dead_code)]
fn shape_test() {
    let line = Line {
        p1: Point { x: 10, y: 15 },
        p2: Point { x: 5, y: -5 },
    };

    let tri = Triangle {
        p1: Point { x: 0, y: 0 },
        p2: Point { x: -5, y: -5 },
        p3: Point { x: 5, y: -5 },
    };

    let sqr = Square {
        p1: Point { x: 2, y: 6 },
        p2: Point { x: 12, y: 10 },
    };

    let ps = vec![
        Point { x: 10, y: 0 },
        Point { x: 7, y: 7 },
        Point { x: 0, y: 10 },
        Point { x: -7, y: 7 },
        Point { x: -10, y: 0 },
        Point { x: -7, y: -7 },
        Point { x: 0, y: -10 },
        Point { x: 7, y: -7 },
    ];

    let pol = Polygon { points: ps };

    println!("--- Polygon ---\n{}", pol.circumference());

    println!("--- Square ---\n{}", sqr.area());
    println!("{}", sqr.circumference());

    println!("--- Triangle ---\n{}\n{}", tri.area(), tri.circumference());

    println!("--- Line ---\n{}", line.get_len());

    let pos = Pos::X(200);
    pos.get_value();
}

#[allow(dead_code)]
fn trait_test() {
    let news = Article {
        author: String::from("lost_script"),
        headline: String::from("10 Most usefull ways to UwU"),
        content: String::from("1. UwU\n2. OwO\n3. TuT\n4. ^w^\n5. :3\n ..."),
    };
    let post = Post {
        username: String::from("lostplayer"),
        content: String::from("Gib bois"),
        repost: false,
    };
    let repost = Post {
        username: String::from("yesman"),
        content: String::from("Gib bois"),
        repost: true,
    };

    notify(&news);
    notify(&post);
    notify(&repost);
}

#[allow(dead_code)]
fn closure_test() {
    println!("{}", implied_type());
    closure_borrow();
    closure_thread();
    closure_count();
}

#[allow(dead_code)]
fn iterator_test() {
    lazy_test();
    map_and_filter();
}

#[allow(dead_code)]
fn spointer_test() {
    custom_spointer_test();
    cons_test();
}

#[allow(dead_code)]
fn concurrency_test() {
    thread_test();
    capture_thread_test();
    thread_channels();
    thread_channels2();
    multi_sender();
    first_mutex();
    multi_thread_mutex();
    // deadlock_test();
}

fn main() {
    // shape_test();
    // why(); // Just to see how super works
    // loop_vec();
    // error_test();
    // trait_test();
    // lifetime_test();
    // closure_test();
    // iterator_test();
    // spointer_test();
    // concurrency_test();
}
