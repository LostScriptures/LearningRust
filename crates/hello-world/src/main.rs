use rand::prelude::*;
use std::cmp::Ordering;
use std::io::Write;

#[allow(unused)]
static PI: f64 = 3.1415926;

fn main() {
    let mut rng = rand::rng();

    let num = rng.random_range(1..=100);

    loop {
        print!("Deine Eingabe: ");
        std::io::stdout().flush().expect("Could not flush stdout");

        let mut input = String::new();

        std::io::stdin()
            .read_line(&mut input)
            .unwrap_or_else(|e| panic!("{e}"));
        let input = match input.trim().parse::<i32>() {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Gib ne Zahl ein!\n{e}");
                continue;
            }
        };

        match input.cmp(&num) {
            Ordering::Less => println!("zu klein"),
            Ordering::Greater => println!("zu groß"),
            Ordering::Equal => {
                println!("SEHR GUT!!!!!!\nDU HAST GEWONNEN OwO");
                break;
            }
        }
    }
}
