mod some_module;

use managing_projects;

use hello::Language;
use hello::casual;

fn main() {
    println!("Running the project executable");
    some_module::mod_func();
    managing_projects::lib_func();
    hello::english();
    casual::english();
    println!("{:?}", Language::new().language_type);
}

// define inline module
mod hello {
    // add enum to module
    #[derive(Debug)]
    pub enum LanguageType {
        Cool
    }

    // add struct to module
    pub struct Language {
        // mark field as public to access from the main module
        pub language_type: LanguageType,
    }

    // add impl to the model
    impl Language {
        pub fn new() -> Language {
            Language { language_type: LanguageType::Cool }
        }
    }

    pub fn english() {
        println!("Hello");
        // call inner module function
        casual::english();
    }

    fn spanish() {
        println!("Hola");
    }

    pub mod casual {
        pub fn english() {
            println!("Hey");
            // call a function using relative path and super
            super::spanish();
            // call a function using absolute path
            crate::hello::spanish();
        }
    }
}
