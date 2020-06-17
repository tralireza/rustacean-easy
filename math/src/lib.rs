//! # Math

/// 2413 Smallest Even Multiple
struct Sol2413 {}

impl Sol2413 {
    pub fn smallest_even_multiple(n: i32) -> i32 {
        println!(":? {}", n << (n & 1));
        if n & 1 == 0 { n } else { 2 * n }
    }
}

/// 2525 Categorize Box According to Criteria
struct Sol2525 {}

impl Sol2525 {
    pub fn categorize_box(length: i32, width: i32, height: i32, mass: i32) -> String {
        match (
            length >= 10000
                || width >= 10000
                || height >= 10000
                || length as i64 * width as i64 * height as i64 >= 1e9 as i64,
            mass >= 100,
        ) {
            (true, true) => "Both",
            (true, false) => "Bulky",
            (false, true) => "Heavy",
            _ => "Neither",
        }
        .to_string()
    }
}

#[cfg(test)]
mod tests;
