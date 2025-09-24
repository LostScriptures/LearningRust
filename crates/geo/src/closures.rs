// Differnt types of closure syntax
// fn  add_one_v1   (x: u32) -> u32 { x + 1 }
// let add_one_v2 = |x: u32| -> u32 { x + 1 };
// let add_one_v3 = |x|             { x + 1 };
// let add_one_v4 = |x|               x + 1  ;

use std::thread;

pub fn implied_type() -> String {
    let closure = |x| x;

    let s = closure(String::from("test")); // Type of x is from now on String

    s
}

pub fn closure_borrow() {
    let mut list = vec![1, 2, 3];

    println!("Before borrow closure: {list:?}");

    let only_borrow = || println!("From closure: {list:?}");
    only_borrow();

    println!("After closure: {list:?}");

    let mut mut_borrow = || list.push(4);
    // No other borrows of list allowed between closure definition and the last time it's called
    mut_borrow();

    println!("After mut borrow: {list:?}");

    list.push(5);

    // mut_borrow(); this would create an error
}

pub fn closure_thread() {
    let list = vec![1, 2, 3];
    println!("Before closure: {list:?}");

    thread::spawn(move || println!("From thread: {list:?}")) // Takes ownership of captured values
        .join()
        .unwrap();

    // println!("After thread: {list:?}"); Would create and error
}

enum Test<T> {
    A(T),
    B,
}

impl<T> Test<T> {
    pub fn test_a<F>(&self, a: F) -> Result<&T, T>
    where
        F: FnOnce() -> T,
    {
        match self {
            Test::A(x) => Ok(x),
            Test::B => Err(a()),
        }
    }
}

// Why did i do this to myself
pub fn closure_test() {
    let x: Test<i32> = Test::B;

    let col = || 12;

    let result = x.test_a(col).unwrap_err();

    println!("{result}");
}
