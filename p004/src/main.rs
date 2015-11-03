extern crate p004;

// $ time cargo run
//      Running `target/debug/p004`
// 906609
//
// real	0m3.721s
// user	0m3.640s
// sys	0m0.092s
fn main() {
    println!("{}", p004::f(100, 999));
}
