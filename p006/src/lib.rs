//! https://projecteuler.net/problem=6
//!
//! The sum of the squares of the first ten natural numbers is,
//! 12 + 22 + ... + 102 = 385
//!
//! The square of the sum of the first ten natural numbers is,
//! (1 + 2 + ... + 10)2 = 552 = 3025
//!
//! Hence the difference between the sum of the squares of the first ten
//! natural numbers and the square of the sum is 3025 − 385 = 2640.
//!
//! Find the difference between the sum of the squares of the first one
//! hundred natural numbers and the square of the sum.

/// # Examples
///
/// ```
/// assert_eq!(p006::f(10), 2640);
/// assert_eq!(p006::f(100), 25164150);
/// ```
///
pub fn f(max_number: i32) -> i32 {
    let sum_squares: i32 =
        (1..(max_number + 1))
        .map(|x| x * x)
        .fold(0, | sum, x | sum + x);

    let sum: i32 = (1..(max_number + 1)).fold(0, | sum, x | sum + x);

    sum * sum - sum_squares
}
