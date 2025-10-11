use config::*;
use display::*;
use to_do::*;

mod config;
mod display;
mod to_do;

fn main() {
    println!("{}", size_of::<Task>());
}
