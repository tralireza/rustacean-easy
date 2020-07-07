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

        let mut played = HashSet::new();

        let (mut player, mut round) = (0, 1);
        while played.insert(player) {
            player = (player + round * k) % n;
            round += 1;
        }

        (0..n)
            .filter(|n| !played.contains(n))
            .map(|n| n + 1)
            .collect()
    }
}

/// 2748 Number of Beautiful Pairs
struct Sol2748 {}

impl Sol2748 {
    pub fn count_beautiful_pairs(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        fn gcd(mut a: i32, mut b: i32) -> i32 {
            if b > a {
                return gcd(b, a);
            }

            while b > 0 {
                (a, b) = (b, a % b);
            }
            a
        }

        let mut fdigs = HashMap::new();

        nums.into_iter()
            .map(|mut n| {
                let (mut fdig, ldig) = (n % 10, n % 10);
                while n > 0 {
                    fdig = n % 10;
                    n /= 10;
                }

                (fdig, ldig)
            })
            .fold(0, |mut pairs, (fdig, ldig)| {
                for (dig, f) in fdigs.iter() {
                    if gcd(*dig, ldig) == 1 {
                        pairs += *f;
                    }
                }

                fdigs.entry(fdig).and_modify(|f| *f += 1).or_insert(1);
                println!("-> {pairs} {fdigs:?}");

                pairs
            })
    }
}

/// 2815 Max Pair Sum in an Array
struct Sol2815 {}

impl Sol2815 {
    pub fn max_sum(nums: Vec<i32>) -> i32 {
        use std::cmp::Reverse;

        let mut nums: Vec<_> = nums
            .iter()
            .map(|&n| {
                let mut key = 0;
                let mut x = n;
                while x > 0 {
                    key = key.max(x % 10);
                    x /= 10;
                }
                (key, n)
            })
            .collect();

        nums.sort_unstable_by_key(|&(key, n)| Reverse((key, n)));
        println!("-> {nums:?}");

        nums.chunk_by(|(k1, _), (k2, _)| k1 == k2)
            .filter(|chunk| chunk.len() > 1)
            .map(|chunk| chunk[0].1 + chunk[1].1)
            .fold(-1, |max_sum, cur_sum| max_sum.max(cur_sum))
    }
}

#[cfg(test)]
mod tests;
