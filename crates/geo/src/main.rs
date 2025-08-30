use crate::shapes::*;

pub mod shapes;
fn main() {
    let sqr = Square {
        p1: Point { x: 2, y: 6 },
        p2: Point { x: 12, y: 10 },
    };

    println!("{}", sqr.area());
    println!("{}", sqr.circumference());
}
