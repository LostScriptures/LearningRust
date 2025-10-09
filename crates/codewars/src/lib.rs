pub mod count_duplicates;
pub mod deadfish;
pub mod dna_strand;
pub mod maskify;
pub mod sort_array;

pub fn digit_square(num: u64) -> u64 {
    num.to_string()
        .drain(..)
        .map(|val| val.to_digit(10).unwrap().pow(2).to_string())
        .collect::<Vec<String>>()
        .concat()
        .parse::<u64>()
        .unwrap()
}
