struct A {
    x: i32,
    y: i32,
}

pub fn test() -> (i32, i32) {
    let a: A = A { x: 10, y: 20 };
    (a.x, a.y)
}
