use super::lifetime_in_struct::ImportantExcerpt;

// here rule #3 is working, if self is present - all output parameters will be with lifetime self
// see lifetime_elision
impl<'a> ImportantExcerpt<'a> {
    fn return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
