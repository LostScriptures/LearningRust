use crate::shapes::*;
use std::vec;

pub mod shapes;

pub fn test() {
    println!("Wrong way around ^^");
}

fn main() {
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

    why(); // Just to see how super works

    println!("--- Polygon ---\n{}", pol.circumference());

    println!("--- Square ---\n{}", sqr.area());
    println!("{}", sqr.circumference());

    println!("--- Triangle ---\n{}\n{}", tri.area(), tri.circumference());

    println!("--- Line ---\n{}", line.get_len());

    let pos = Pos::X(200);
    pos.get_value();
}
