// Definition
pub trait Test<T> {
    fn foo(&self, bar: T) -> (&T, T);
}

pub trait Test2 {
    type Item;
    fn foo2(&self) -> &Self::Item;
}

struct TestStruct<T> {
    val: T,
}

// Implementation
// This is a generic implementation for any type T that implemts Copy
impl<T> Test<T> for TestStruct<T>
where
    T: Copy,
{
    fn foo(&self, bar: T) -> (&T, T) {
        (&self.val, bar)
    }
}

// This is an implementation specifically for u32
impl<T> Test2 for TestStruct<T> {
    type Item = T;
    fn foo2(&self) -> &Self::Item {
        &self.val
    }
}

// Conflicting implementation
// impl Test2 for TestStruct {
//     type Item = i32;
//     fn foo() -> Self::Item {

//     }
// }

fn _do_stuff<T: Test2>(x: &T) {
    let _val = x.foo2();
}

#[allow(dead_code)]
pub fn asspciated_test() {
    let a: TestStruct<i32> = TestStruct { val: 10 };
    let a2: TestStruct<u32> = TestStruct { val: 100 };

    let (b, c) = a.foo(11);
    let d = a2.foo2();

    a.foo2();

    println!("{} {}", b, c);
    println!("{}", d);

    let t = vec![1, 2, 3];
    let _a = t.iter();
}