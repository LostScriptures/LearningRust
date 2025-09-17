use crate::shapes::*;
#[allow(unused_imports)]
use crate::vecs::*;
use std::vec;

pub mod errors;
pub mod shapes;
pub mod vecs;
pub mod generics;

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

fn main() {
    // shape_test();

    why(); // Just to see how super works

    // loop_vec();

    // error_test();
}
