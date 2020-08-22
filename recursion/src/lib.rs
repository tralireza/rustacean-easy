//! # Recursion

/// 3483
struct Sol3483 {}

impl Sol3483 {
    pub fn total_numbers(digits: Vec<i32>) -> i32 {
        use std::collections::HashSet;

        fn search(r: i32, p: i32, digits: &[i32], usage: &mut [bool], pset: &mut HashSet<i32>) {
            if r == 3 {
                if p & 1 == 0 {
                    pset.insert(p);
                }
                return;
            }

            for (i, &d) in digits.iter().enumerate() {
                if !usage[i] && (d > 0 || d == 0 && p > 0) {
                    usage[i] = true;
                    search(r + 1, 10 * p + d, digits, usage, pset);
                    usage[i] = false;
                }
            }
        }

        let mut pset = HashSet::new();
        let mut usage = vec![false; digits.len()];

        search(0, 0, &digits, &mut usage, &mut pset);

        println!("-> {pset:?}");
        pset.len() as _
    }
}

#[cfg(test)]
mod tests;
