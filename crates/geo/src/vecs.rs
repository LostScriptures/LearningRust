use std::vec;

pub fn loop_vec() {
    let mut v: Vec<i32> = Vec::new();

    for i in 0..100 {
        v.push(i);
    }

    for e in &v {
        if e % 2 == 0 {
            print!("{e} ");
        }
    }
    println!("");
}
