extern crate p002;

// time cargo run
//     Running `target/debug/p002`
// 4613732
//
// real	0m0.194s
// user	0m0.120s
// sys	0m0.080s
fn main() {
    println!("{}", p002::f(4000000));
}
