extern crate p002;

// $ time cargo run
//      Running `target/debug/p001`
// 233168
//
// real	0m0.209s
// user	0m0.104s
// sys	0m0.108s
fn main() {
    println!("{}", p002::f(4000000));
}
