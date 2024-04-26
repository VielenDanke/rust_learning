/// To cal methods with the same name we need to point out what method we should use
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

pub fn example_methods() {
    let person = Human;
    person.fly();

    // to call different struct methods we are using explicit type::method declaration
    Wizard::fly(&person);
    Pilot::fly(&person);
    person.fly()
}

/// Call functions

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

/// we need to declare explicitly what struct function we want to call by using <Dog as Animal>::call()
pub fn example_functions() {
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}
