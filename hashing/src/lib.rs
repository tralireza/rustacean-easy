//! # Hashing

/// 2395 Find Subarrays With Equal Sum
struct Sol2395 {}

impl Sol2395 {
    pub fn find_subarrays(nums: Vec<i32>) -> bool {
        use std::collections::HashSet;

        let mut sset = HashSet::new();
        nums.windows(2)
            .filter(|w| {
                if sset.contains(&w.iter().sum::<i32>()) {
                    true
                } else {
                    sset.insert(w.iter().sum::<i32>());
                    false
                }
            })
            .count()
            > 0
    }
}

/// 2451 Odd String Difference
struct Sol2451 {}

impl Sol2451 {
    pub fn odd_string(words: Vec<String>) -> String {
        let mut diffs = vec![];
        words.iter().take(2).for_each(|w| {
            let dw: Vec<_> = w
                .as_bytes()
                .windows(2)
                .map(|w| b'a' + w[1] - w[0])
                .collect();
            diffs.push(dw);
        });

        let equal = diffs[0]
            .iter()
            .zip(diffs[1].iter())
            .all(|(d1, d2)| d1 == d2);

        println!("-> {equal} {diffs:?}");

        let word0 = words[0].clone();
        let word1 = words[1].clone();
        for word in words.into_iter().skip(2) {
            let dw: Vec<_> = word
                .as_bytes()
                .windows(2)
                .map(|w| b'a' + w[1] - w[0])
                .collect();

            println!("-> {word:?} {dw:?}");

            return match (
                equal,
                diffs[0].iter().zip(dw.iter()).all(|(d1, d2)| d1 == d2),
            ) {
                (false, true) => word1,
                (false, false) => word0,
                (true, false) => word,
                _ => continue,
            };
        }

        "".to_string()
    }
}

/// 2500 Delete Greatest Value in Each Row
struct Sol2500 {}

impl Sol2500 {
    pub fn delete_greatest_value(grid: Vec<Vec<i32>>) -> i32 {
        use std::collections::BinaryHeap;

        let m = grid[0].len();

        let mut qs = vec![];
        for row in grid {
            qs.push(BinaryHeap::from(row));
        }

        println!("-> {qs:?}");

        let mut g = 0;
        for _ in 0..m {
            let mut gs = vec![];
            for q in &mut qs {
                gs.push(q.pop().unwrap());
            }
            g += gs.into_iter().max().unwrap();
        }

        g
    }
}

/// 2506 Count Pairs Of Similar Strings
struct Sol2506 {}

impl Sol2506 {
    pub fn similar_pairs(words: Vec<String>) -> i32 {
        use std::collections::HashMap;

        let mut m_freqs = HashMap::new();
        println!(
            ":? {}",
            words
                .iter()
                .map(|w| {
                    let mut mask = 0;
                    for c in w.as_bytes() {
                        mask |= 1 << (c - b'a');
                    }
                    mask
                })
                .map(|m| {
                    let count = *m_freqs.get(&m).unwrap_or(&0);
                    m_freqs.insert(m, count + 1);
                    count
                })
                .sum::<i32>()
        );
        println!("-> Bitwise Frequencey: {m_freqs:?}");

        let mut freqs = HashMap::new();
        words
            .iter()
            .map(|w| {
                let mut a = [false; 26];
                for c in w.as_bytes() {
                    a[(c - b'a') as usize] = true;
                }
                a
            })
            .map(|a| {
                let count = *freqs.get(&a).unwrap_or(&0);
                freqs.insert(a, count + 1);
                count
            })
            .sum::<i32>()
    }
}

/// 2682 Find the Losers of the Circular Game
struct Sol2682 {}

impl Sol2682 {
    pub fn circular_game_losers(n: i32, k: i32) -> Vec<i32> {
        use std::collections::HashSet;

        let mut cur = 0;
        let mut played = HashSet::new();
        played.insert(cur);

        for round in 1.. {
            let next = (cur + round * k) % n;
            if !played.insert(next) {
                break;
            }

            cur = next;
        }

        (0..n)
            .filter(|n| !played.contains(n))
            .map(|n| n + 1)
            .collect()
    }
}

#[cfg(test)]
mod tests;
