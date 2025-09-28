use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
};

// let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))));
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

pub struct MyBox<T>(T)
where
    T: Display;

impl<T> MyBox<T>
where
    T: Display,
{
    pub fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T>
where
    T: Display,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for MyBox<T>
where
    T: Display,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Drop for MyBox<T>
where
    T: Display,
{
    fn drop(&mut self) {
        println!("Dropping: {} !", self.0);
    }
}

pub fn hello(name: &mut String, new_name: String) {
    println!("Hello, {name}. Changes To {new_name}");
    *name = new_name;
}

pub fn custom_spointer_test() {
    let mut m = MyBox::new(String::from("Rust"));
    let new = String::from("UwU");
    hello(&mut m, new);
    // drop(m); Would relese the value before the println
    println!("{}", *m);
}
