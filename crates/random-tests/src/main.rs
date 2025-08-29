struct Dummy {}

impl Dummy {
    // Funktion to print a pyramid with the height of n
    fn pyramid(&self, n: usize) {
        for i in 0..n {
            let spaces = " ".repeat(n - i - 1);
            let stars = "*".repeat(2 * i + 1);
            println!("{}{}", spaces, stars);
        }
    }
}
fn main() {
    let new = Dummy {};
    new.pyramid(10);

    
    let s = "20";

    let i = s.parse::<i32>();


}
