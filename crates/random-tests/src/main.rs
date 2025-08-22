pub mod test;
use crate::test::A as B;

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

    let a: B = B { x: 12, y: 13 };
}
