//! https://projecteuler.net/problem=7
//!
//! By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see
//! that the 6th prime is 13.
//!
//! What is the 10 001st prime number?

use std::collections::HashMap;

/// # Examples
///
/// ```
/// assert_eq!(p007::eratosthenes(15), [2, 3, 5, 7, 11, 13]);
/// assert_eq!(p007::eratosthenes(25), [2, 3, 5, 7, 11, 13, 17, 19, 23]);
/// ```
///
pub fn eratosthenes(max_number: i32) -> Vec<i32> {
    let mut primes: Vec<i32> = vec![2];
    let mut not_primes: HashMap<i32, i32> = HashMap::new();

    let mut i: i32 = 1;
    while i < max_number {
        i += 2;

        if not_primes.contains_key(&i) {
            continue;
        }

        primes.push(i);

        let mut j: i32 = i + i;
        while j <= max_number {
            if !not_primes.contains_key(&j) {
                not_primes.insert(j, 1);
            }

            j = j + i;
        }
    }

    return primes
}

/// # Examples
///
/// ```
/// assert_eq!(p007::f(6), 13);
/// assert_eq!(p007::f(10001), 104743);
/// ```
///
pub fn f(nth_prime_number: i32) -> i32 {
    let max_number: i32 = nth_prime_number * 20;
    eratosthenes(max_number)[nth_prime_number as usize - 1]
}
