pub struct A {
    pub x: i32,
    pub y: i32,
}

trait edit {
    fn add(&self, x: i32, y: i32) -> A;
    fn sub(&self, x: i32, y: i32) -> A;
    fn div(&self, x: f64, y: f64) -> A;
    fn mul(&self, x: f64, y: f64) -> A;
}

impl edit for A {
    fn add(&self, x: i32, y: i32) -> A {
        A {
            x: (self.x + x),
            y: (self.y + y),
        }
    }

    fn sub(&self, x: i32, y: i32) -> A {
        A {
            x: (self.x - x),
            y: (self.y - y),
        }
    }

    fn div(&self, x: f64, y: f64) -> A {
        let fx: f64 = self.x as f64;
        let fy: f64 = self.y as f64;

        A {
            x: (fx / x) as i32,
            y: (fy / y) as i32,
        }
    }

    fn mul(&self, x: f64, y: f64) -> A {
        let fx: f64 = self.x as f64;
        let fy: f64 = self.y as f64;

        A {
            x: (fx * x) as i32,
            y: (fy * y) as i32,
        }
    }
}
