pub trait Draw {
    fn draw(&self);
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Draw a button with width {} and height {}. Label {}.", self.width, self.height, self.label);
    }
}

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("Draw a select box with width {} and height {}. Options {:?}.", self.width, self.height, self.options);
    }
}

pub struct Screen {
    // if we know that Vec<> will include only certain type we could use T: Draw
    // but if it could be TextField, Button and any other types implemented Draw we need to use Box<dyn Draw> because size is unknown
    // in compile time
    // Dynamic typing is slower than static typing
    // if we want our code to be more flexible - use dynamic typing
    // if we want our code to be faster - use static typing
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub fn run_example() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}
