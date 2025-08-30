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
pub struct Polygon {
    pub points: Vec<Point>,
}

pub trait Geo {
    fn area(&self) -> i32;
    fn circumference(&self) -> i32;
}

impl Point {
    fn get_dist(&self, target_point: &Point) -> f32 {
        let vec = Point {
            x: target_point.x - self.x,
            y: target_point.y - self.y,
        };
        f32::sqrt((vec.x.pow(2) + vec.y.pow(2)) as f32)
    }
    fn get_minmax(&self, p2: &Point) -> (Point, Point) {
        let min_point = Point {
            x: cmp::min(self.x, p2.x).abs(),
            y: cmp::min(self.y, p2.y).abs(),
        };
        let max_point = Point {
            x: cmp::max(self.x, p2.x).abs(),
            y: cmp::max(self.y, p2.y).abs(),
        };
        (min_point, max_point)
    }
}

impl Geo for Square {
    fn area(&self) -> i32 {
        let (min_point, max_point) = self.p1.get_minmax(&self.p2);
        (max_point.x - min_point.x) * (max_point.y - min_point.y)
    }
    fn circumference(&self) -> i32 {
        let (min_point, max_point) = self.p1.get_minmax(&self.p2);
        ((max_point.x - min_point.x) * 2) + ((max_point.y - min_point.y) * 2)
    }
}

impl Geo for Triangle {
    fn area(&self) -> i32 {
        let exp1 = self.p1.x * (self.p2.y - self.p3.y);
        let exp2 = self.p2.x * (self.p3.y - self.p1.y);
        let exp3 = self.p3.x * (self.p1.y - self.p2.y);

        (1 / 2) * (exp1 + exp2 + exp3).abs()
    }
    fn circumference(&self) -> i32 {
        let mut dist: f32 = 0.0;

        dist += self.p1.get_dist(&self.p2);
        dist += self.p2.get_dist(&self.p3);
        dist += self.p3.get_dist(&self.p1);
        dist as i32
    }
}
