use std::cmp;
pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub struct Line {
    pub p1: Point,
    pub p2: Point,
}

pub struct Square {
    pub p1: Point,
    pub p2: Point,
}
pub struct Triangle {
    pub p1: Point,
    pub p2: Point,
    pub p3: Point,
}

pub trait Geo {
    fn area(&self) -> i32;
    fn circumference(&self) -> i32;
    fn get_minmax(&self) -> (Point, Point);
}

impl Geo for Square {
    fn get_minmax(&self) -> (Point, Point) {
        let min_point = Point {
            x: cmp::min(self.p1.x, self.p2.x).abs(),
            y: cmp::min(self.p1.y, self.p2.y).abs(),
        };
        let max_point = Point {
            x: cmp::max(self.p1.x, self.p2.x).abs(),
            y: cmp::max(self.p1.y, self.p2.y).abs(),
        };
        (min_point, max_point)
    }
    fn area(&self) -> i32 {
        let (min_point, max_point) = self.get_minmax();
        (max_point.x - min_point.x) * (max_point.y - min_point.y)
    }
    fn circumference(&self) -> i32 {
        let (min_point, max_point) = self.get_minmax();
        ((max_point.x - min_point.x) * 2) + ((max_point.y - min_point.y) * 2)
    }
}
