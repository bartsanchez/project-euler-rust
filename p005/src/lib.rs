//! https://projecteuler.net/problem=5
//!
//! 2520 is the smallest number that can be divided by each of the numbers
//! from 1 to 10 without any remainder.
//!
//! What is the smallest positive number that is evenly divisible by all of
//! the numbers from 1 to 20?

/// # Examples
///
/// ```
/// assert_eq!(p005::f(&(2..11).collect()), 2520);
/// assert_eq!(p005::f(&(2..21).collect()), 232792560);
/// ```
///
pub fn f(vec: &Vec<i32>) -> i32 {
    let bigger: i32 = *vec.iter().max().unwrap();
    let mut result: i32 = 1;

    'inner: for i in (1..).map(|x| x * bigger) {
        result = i;
        for j in vec {
            if result % j != 0 {
                continue 'inner;
            }
        }
        break;
    }
    result
}
