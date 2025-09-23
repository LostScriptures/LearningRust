#[allow(unused_imports)]
use crate::vecs::*;
#[allow(unused_imports)]
use crate::{
    generics::{Article, Post, lifetime_test, notify},
    shapes::*,
};
use std::vec;

pub mod errors;
pub mod generics;
pub mod shapes;
pub mod vecs;

pub fn test() {
    println!("Wrong way around ^^");
}

pub fn error_test() {
    match errors::read_username() {
        Ok(name) => println!("Username: {name}"),
        Err(e) => panic!("An Error ocurred: {e:?}"),
    }
}

pub fn shape_test() {
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

pub fn trait_test() {
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

// Funktion to print a pyramid with the height of n
pub fn pyramid(n: usize) {
    for i in 0..n {
        let spaces = " ".repeat(n - i - 1);
        let stars = "*".repeat(2 * i + 1);
        println!("{}{}", spaces, stars);
    }
}
