use crate::List::{Cons, Nil};
use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
    rc::Rc,
};

// let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))));
pub enum List {
    Cons(i32, Rc<List>),
    Nil,
}

impl List {
    pub fn print(&self) {
        if let Cons(val, next) = self {
            print!("{} ", val);
            next.print();
        } else {
            println!("");
        }
    }
}

pub fn cons_test() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("Refernce Count: {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("Refernce Count: {}", Rc::strong_count(&a));
    let c = Cons(4, Rc::clone(&a));

    print!("A: ");
    a.print();
    println!("Refernce Count: {}", Rc::strong_count(&a));
    {
        print!("B: ");
        b.print();
        println!("Refernce Count: {}", Rc::strong_count(&a));
    }
    print!("C: ");
    c.print();
    println!("Refernce Count: {}", Rc::strong_count(&a));
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
