//! # String

/// 2399 Check Distances Between Same Letters
struct Sol2399 {}

impl Sol2399 {
    pub fn check_distances(s: String, distance: Vec<i32>) -> bool {
        let mut starts = [None; 26];

        s.as_bytes()
            .iter()
            .enumerate()
            .all(|(i, chr)| match starts[(chr - b'a') as usize] {
                None => {
                    starts[i] = Some(i);
                    true
                }
                Some(start) => i == start + 1 + distance[(chr - b'a') as usize] as usize,
            })
    }
}

#[cfg(test)]
mod tests;
