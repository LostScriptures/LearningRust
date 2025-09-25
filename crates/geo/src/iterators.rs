pub fn lazy_test() {
    let new = vec![1, 2, 3];

    let new_iter = new.iter();

    for val in new_iter {
        print!("{val} ");
    }
    println!("");
}

pub fn map_and_filter() {
    let new = vec![1, 2, 3];

    println!("List: {new:?}");

    let changed_new: Vec<i32> = new.into_iter().map(|v| v + 1).collect();

    println!("After map: {changed_new:?}");

    let filtered_new: Vec<i32> = changed_new.into_iter().filter(|v| v % 2 == 0).collect();

    println!("After filter: {filtered_new:?}")
}
