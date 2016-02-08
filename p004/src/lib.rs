//! https://projecteuler.net/problem=4
//!
//! A palindromic number reads the same both ways. The largest palindrome
//! made from the product of two 2-digit numbers is 9009 = 91 × 99.
//!
//! Find the largest palindrome made from the product of two 3-digit numbers.

/// # Examples
///
/// ```
/// assert_eq!(p004::is_palindrome("9009"), true);
/// assert_eq!(p004::is_palindrome("90809"), true);
/// assert_eq!(p004::is_palindrome("12345"), false);
/// ```
///
pub fn is_palindrome(s: &str) -> bool {
    let v: Vec<char> = s.chars().collect();
    let to = v.len() / 2;

    for i in 0..to {
        let start_index = i;
        let end_index = v.len() - i - 1;

        if v[start_index] != v[end_index] {
            return false
        }
    }
    true
}

/// # Examples
///
/// ```
/// assert_eq!(p004::f(10, 99), 9009);
/// assert_eq!(p004::f(100, 999), 906609)
/// ```
///
pub fn f(from: i32, to: i32) -> i32 {
    let mut result: i32 = 0;

    for a in from..to + 1 {
        for b in from..to + 1 {
            let product: i32 = a * b;

            if is_palindrome(&product.to_string()) && product > result {
                result = product
            }
        }
    }
    result
}
