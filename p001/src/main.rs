extern crate p001;

// $ time cargo run
//      Running `target/debug/p001`
// 233168
//
// real	0m0.209s
// user	0m0.104s
// sys	0m0.108s
fn main() {
    println!("{}", p001::f(1000));
}
