/*
The behavior of panic could be changed
To tell Rust not to clean stack and memory after panic appeared, write:
    [profile.release]
    panic = 'abort'
In that case the memory should be cleaned up by OS

By default Rust will traverse all the stack and clean everything up

We can set up RUST_BACKTRACE=1 to display trace calls
 */

fn crash_and_burn() {
    panic!("Crash and Burn");
}

fn reverse_trace_panic() {
    let v = vec![1, 2, 3];

    v[99]; // code will produce panic!
}
