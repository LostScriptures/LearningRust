pub mod test;
use crate::test::A;

fn main() {
    // Tests
    let mut s = String::from("Hello, world!");
    let rs = &s;

    println!("{rs}");

    {
        let ms = &mut s;

        ms.push_str(" it's me :)");
    }

    let ss = &s;
    println!("{ss}");

    let mut a: A = A { x: 12, y: 13 };

    println!("B: {} | {}", a.x, a.y);

    a = a.add(10, 10);

    println!("B: {} | {}", a.x, a.y);
}
