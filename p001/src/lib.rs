//! https://projecteuler.net/problem=1
//!
//! If we list all the natural numbers below 10 that are multiples of 3 or 5,
//! we get 3, 5, 6 and 9. The sum of these multiples is 23.
//!
//! Find the sum of all the multiples of 3 or 5 below 1000.

pub fn f(max: i32) -> i32 {
    //! # Examples
    //!
    //! ```
    //! use p001::f;
    //!
    //! assert_eq!(f(10), 23);
    //! assert_eq!(f(1000), 233168);
    //!
    (1..max)
        .filter(|&x| x % 3 == 0 || x % 5 == 0)
        .fold(0, |sum, x| sum + x)
}
