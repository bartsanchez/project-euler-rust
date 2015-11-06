extern crate p005;

// $ time cargo run
//      Running `target/debug/p005`
// 232792560
//
// real	0m4.624s
// user	0m4.524s
// sys	0m0.108s
fn main() {
    println!("{}", p005::f(&(2..21).collect()));
}
