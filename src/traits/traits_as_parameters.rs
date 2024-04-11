use crate::traits::create_and_implement_trait::Summary;

// pass trait as a parameter
pub fn notify(item: &impl Summary<Item = String>) {
    println!("Breaking news! {}", item.summarize());
}
