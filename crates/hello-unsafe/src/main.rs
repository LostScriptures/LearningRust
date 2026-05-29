unsafe extern "C" {
    safe fn abs(input: i32) -> i32; // Promises rust that abs() is safe (not a guarante by rust)
}

use std::slice;

static mut COUNTER: u32 = 0;

/// SAFETY: Calling this from more than a single thread at a time is undefined
/// behavior, so you *must* guarantee you only call it from a single thread at a time
unsafe fn add_to_counter(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

// This is how to make a Rust function accessable to external C Code
#[unsafe(no_mangle)]
pub extern "C" fn call_from_C() {
    println!("Just called a Rust function");
}

unsafe trait Foo {
    unsafe fn add_counter(&self);
}

unsafe impl Foo for u32 {
    unsafe fn add_counter(&self) {
        unsafe { COUNTER += self }
    }
}

#[allow(dead_code)]
fn unsafe_trait_test() {
    let a: u32 = 5;
    unsafe {
        a.add_counter();
        println!("COUNTER: {}", *(&raw const COUNTER))
    }
}

#[allow(dead_code)]
fn unsafe_pointers() {
    let mut num = 5;

    // Raw pointers allow mutable and immutable references at the same time
    let r1 = &raw const num; // Still safe
    let r2 = &raw mut num; // still safe

    unsafe {
        println!("r1: {}", *r1);
        println!("r2: {}", *r2);
    }

    let address = 0x412345usize;
    let _r = address as *const i32;
}

// Still requires unsafe {} block for unsafe operations but marks the funtion as unsafe.
fn split_at_mut<T>(values: &mut [T], mid: usize) -> (&mut [T], &mut [T]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

#[allow(dead_code)]
fn unsafe_fn_test() {
    let mut data = vec![1, 2, 3, 4, 5, 6];
    let text = "Test String";

    let mut text = text.split("").collect::<Vec<&str>>();

    let (a, b) = split_at_mut(&mut data, 3);
    let (c, d) = split_at_mut(&mut text, 5);

    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);
    println!("{:?}", d);
}

#[allow(dead_code)]
fn extern_test() {
    println!("Absolut value of -3 from C: {}", abs(-3));
}

#[allow(dead_code)]
fn static_test() {
    unsafe {
        add_to_counter(3);
        println!("COUNTER: {}", *(&raw const COUNTER));
    }
}

fn main() {
    // unsafe_pointers();
    // unsafe { unsafe_funtion() }
    // unsafe_fn_test();
    // extern_test();
    // unsafe_trait_test();
    // static_test();
}
