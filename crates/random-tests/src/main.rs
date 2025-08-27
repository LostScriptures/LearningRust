use std::vec;

fn main() {
    pyramid(20);
}

fn pyramid(n: usize) {
    for i in 0..n {
        let spaces = " ".repeat(n - i - 1);
        let stars = "*".repeat(2 * i + 1);
        println!("{}{}", spaces, stars);
    }
}
