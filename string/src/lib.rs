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

/// 2409 Count Days Spent Together
struct Sol2409 {}

impl Sol2409 {
    pub fn count_days_together(
        arrive_alice: String,
        leave_alice: String,
        arrive_bob: String,
        leave_bob: String,
    ) -> i32 {
        let mut days = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        for m in 1..days.len() {
            days[m] += days[m - 1];
        }
        println!("-> {days:?}");

        let mut data = [[[0i32; 3]; 2]; 2];
        for (p, p_data) in [[arrive_alice, leave_alice], [arrive_bob, leave_bob]]
            .iter()
            .enumerate()
        {
            for (x, date_str) in p_data.iter().enumerate() {
                for (i, part) in date_str.split('-').enumerate() {
                    data[p][x][i] = part.parse().unwrap();
                }
                data[p][x][2] = days[data[p][x][0] as usize - 1] + data[p][x][1];
            }
        }
        println!("-> {data:?}");

        0.max(data[0][1][2].min(data[1][1][2]) - data[0][0][2].max(data[1][0][2]) + 1)
    }
}

/// 2423 Remove Letters To Equalize Frequency
struct Sol2423 {}

impl Sol2423 {
    pub fn equal_frequency(word: String) -> bool {
        let mut freqs = [0; 26];
        for chr in word.as_bytes() {
            freqs[(chr - b'a') as usize] += 1;
        }

        for x in 0..26 {
            if freqs[x] > 0 {
                freqs[x] -= 1;

                if freqs
                    .iter()
                    .filter(|f| **f > 0)
                    .collect::<Vec<_>>()
                    .windows(2)
                    .all(|w| w[0] == w[1])
                {
                    return true;
                }

                freqs[x] += 1;
            }
        }

        false
    }
}

/// 2437 Number of Valid Clock Times
struct Sol2437 {}

impl Sol2437 {
    pub fn count_time(time: String) -> i32 {
        let mut bforce = 0;
        for hour in 0..24 {
            for minute in 0..60 {
                let ctime = format!("{:02}:{:02}", hour, minute);
                if time
                    .chars()
                    .zip(ctime.chars())
                    .all(|(t, c)| t == '?' || t == c)
                {
                    bforce += 1;
                }
            }
        }
        println!(":? {bforce}");

        let mut count = 1;

        if let [h1, h2, _, m1, m2] = time.chars().collect::<Vec<_>>()[..] {
            count *= match (h1, h2) {
                ('?', '?') => 24,
                ('?', '0'..='3') => 3,
                ('?', _) => 2,
                ('2', '?') => 4,
                (_, '?') => 10,
                _ => 1,
            };

            count *= if m1 == '?' { 6 } else { 1 };
            count *= if m2 == '?' { 10 } else { 1 };
        }

        count
    }
}

/// 2515 Shortest Distance to Target String in a Circular Array
struct Sol2515 {}

impl Sol2515 {
    pub fn closest_target(words: Vec<String>, target: String, start_index: i32) -> i32 {
        let start_index = start_index as usize;

        println!(
            ":? {}",
            words[start_index..]
                .iter()
                .chain(words[..start_index].iter())
                .enumerate()
                .filter(|(_, w)| **w == target)
                .fold(usize::MAX, |dist, (i, _)| {
                    if 2 * i > words.len() {
                        dist.min(words.len() - i)
                    } else {
                        dist.min(i)
                    }
                })
        );

        let mut dist = usize::MAX;
        for i in (start_index..words.len()).chain(0..start_index) {
            if words[i] == target {
                let mut d_cur = (words.len() + i - start_index) % words.len();
                if 2 * d_cur > words.len() {
                    d_cur = words.len() - d_cur;
                }
                dist = dist.min(d_cur);
            }
        }

        if dist == usize::MAX {
            return -1;
        }
        dist as _
    }
}

/// 2609 Find the Longest Balanced Substring of a Binary String
struct Sol2609 {}

impl Sol2609 {
    pub fn find_the_longest_balanced_substring(s: String) -> i32 {
        let chars: Vec<_> = s.chars().collect();

        (0..chars.len()).fold(0, |longest, start| {
            let z = chars
                .iter()
                .skip(start)
                .take_while(|chr| **chr == '0')
                .count();
            let o = chars
                .iter()
                .skip(start + z)
                .take_while(|chr| **chr == '1')
                .count();

            if z <= o && longest < 2 * z {
                2 * z
            } else {
                longest
            }
        }) as _
    }
}

/// 2697 Lexicographically Smallest Palindrome
struct Sol2697 {}

impl Sol2697 {
    pub fn make_smallest_palindrome(s: String) -> String {
        s.chars()
            .zip(s.chars().rev())
            .map(|(l, r)| if l > r { r } else { l })
            .collect()
    }
}

#[cfg(test)]
mod tests;
