extern crate p007;

// $ time cargo run
//      Running `target/debug/p007`
// 104743
//
// real	0m1.789s
// user	0m1.692s
// sys	0m0.088s
fn main() {
    println!("{}", p007::f(10001));
}
