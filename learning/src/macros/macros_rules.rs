pub fn example() {
    let v: Vec<u32> = vec![1, 2, 3];

    /*
    #[macro_export] // macro should be accessible when the crate with the macro is added to view area
    macro_rules! vec {
        ( $( $x:expr ),* ) => { // could be many patterns to match the expression
            {
                let mut temp_vec = Vec::new();
                $(
                    temp_vec.push($x);
                )*
                temp_vec
            }
        };
    }
     */
}
