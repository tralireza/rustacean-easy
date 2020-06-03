//! # String

/// 2399 Check Distances Between Same Letters
struct Sol2399 {}

impl Sol2399 {
    pub fn check_distances(s: String, distance: Vec<i32>) -> bool {
        let mut ends = [0; 26];

        let s = s.as_bytes();
        for (i, chr) in s.iter().enumerate() {
            ends[(*chr - b'a') as usize] = i;
        }

        s.iter()
            .enumerate()
            .filter(|(i, chr)| *i < ends[(*chr - b'a') as usize])
            .all(|(_, &chr)| {
                if ends[(chr - b'a') as usize] <= distance[(chr - b'a') as usize] as usize {
                    false
                } else {
                    s[ends[(chr - b'a') as usize] - distance[(chr - b'a') as usize] as usize - 1]
                        == chr
                }
            })
    }
}

#[cfg(test)]
mod tests;
