pub struct Point {
    x: i32,
    y: i32,
}

pub struct Line {
    p1: Point,
    p2: Point,
}

pub struct Square {
    p1: Point,
    p2: Point,
}
pub struct Triangle {
    p1: Point,
    p2: Point,
    p3: Point,
}

pub trait Geo {
    fn area(&self) -> f32;
    fn circumference(&self) -> f32;
}

impl Geo for Square {
    fn area(&self) -> f32 {}
    fn circumference(&self) -> f32 {}
}
