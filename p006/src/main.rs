extern crate p006;

//$ time cargo run
//      Running `target/debug/p006`
// 25164150
//
// real	0m0.185s
// user	0m0.084s
// sys	0m0.112s
fn main() {
    println!("{}", p006::f(100));
}
