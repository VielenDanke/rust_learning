pub fn calculate_product_cost(first_product: u32, second_product: u32) -> u32 {
    first_product + second_product
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_product_cost() {
        let first_coast = 8;
        let second_coast = 19;
        assert_eq!(first_coast + second_coast, calculate_product_cost(first_coast, second_coast));
    }
}
