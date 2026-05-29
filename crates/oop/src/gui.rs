pub trait Draw {
    fn draw(&self);
}

// Can contain different types implementing the Draw trait
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

// Could only contain ONE type implemting the Draw trait
// pub struct Screen<T: Draw> {
//     pub components: Vec<T>,
// }

impl Screen {
    pub fn run(&self) {
        self.components
            .iter()
            .for_each(|component| component.draw());
    }
}

// A type implementing Draw
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!(
            "Button( W: {} H: {} Txt: {} )",
            self.width, self.height, self.label
        )
    }
}

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        print!("SelectBox( W: {} H: {} Opt: ", self.width, self.height);
        self.options.iter().for_each(|option| print!("{option} "));
        println!(")");
    }
}
