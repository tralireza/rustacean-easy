//! # Math

/// 2413 Smallest Even Multiple
struct Sol2413 {}

impl Sol2413 {
    pub fn smallest_even_multiple(n: i32) -> i32 {
        if n & 1 == 0 { n } else { 2 * n }
    }
}

#[cfg(test)]
mod tests;
