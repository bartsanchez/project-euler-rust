//! https://projecteuler.net/problem=3
//!
//! The prime factors of 13195 are 5, 7, 13 and 29.
//!
//! What is the largest prime factor of the number 600851475143 ?

/// # Examples
///
/// ```
/// assert_eq!(p003::f(13195), 29);
/// assert_eq!(p003::f(600851475143), 6857)
/// ```
///
pub fn f(n: i64) -> i64 {
    let mut result: i64 = n;
    let mut i: i64 = 2;

    while result / i != 1 {
        if result % i == 0 {
            result /= i;
        }
        i += 1;
    }
    result
}
