//! # Recursion

/// 3483
struct Sol3483 {}

impl Sol3483 {
    pub fn total_numbers(mut digits: Vec<i32>) -> i32 {
        use std::collections::HashSet;

        fn dsearch(r: i32, p: i32, digits: &mut [i32], pset: &mut HashSet<i32>) {
            if r == 3 {
                if p & 1 == 0 {
                    pset.insert(p);
                }
                return;
            }

            for i in 0..digits.len() {
                if digits[i] > 0 || digits[i] == 0 && p > 0 {
                    let d = digits[i];
                    digits[i] = -1;
                    dsearch(r + 1, 10 * p + d, digits, pset);
                    digits[i] = d;
                }
            }
        }

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

        dsearch(0, 0, &mut digits, &mut pset);
        println!(":? {pset:?}   {}", pset.len());

        pset.clear();
        search(0, 0, &digits, &mut usage, &mut pset);

        println!("-> {pset:?}");
        pset.len() as _
    }
}

#[cfg(test)]
mod tests;
