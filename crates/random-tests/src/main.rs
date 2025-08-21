pub mod test;

fn main() {
    let mut s = String::from("Hello, world!");

    let rs = &s;
    println!("{rs}");

    let (a, b) = test::test();

    {
        let ms = &mut s;

        ms.push_str(" it's me :)");
    }

    let ss = &s;
    println!("{ss}");
    print!("{a} | ");
    print!("{b}\n");
}
