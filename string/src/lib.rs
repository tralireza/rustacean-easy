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

/// 2744 Find Maximum Number of String Pairs
struct Sol2744 {}

impl Sol2744 {
    pub fn maximum_number_of_string_pairs(words: Vec<String>) -> i32 {
        use std::collections::HashMap;

        let mut frevs = HashMap::new();
        words.iter().fold(0, |mut pairs, w| {
            let mut chars: Vec<_> = w.chars().collect();
            if let Some(f) = frevs.get_mut(&chars) {
                pairs += *f;
                *f += 1;
            } else {
                chars.reverse();
                frevs.insert(chars.clone(), 1);
            }
            println!("-> {w:?} {frevs:?}");

            pairs
        })
    }
}

/// 2788 Split String by Separator
struct Sol2788 {}

impl Sol2788 {
    pub fn split_words_by_separator(words: Vec<String>, separator: char) -> Vec<String> {
        words
            .iter()
            .flat_map(|w| w.split(separator))
            .filter(|w| !w.is_empty())
            .map(|w| w.to_string())
            .collect()
    }
}

/// 2833 Furthest Point From Origin
struct Sol2833 {}

impl Sol2833 {
    pub fn furthest_distance_from_origin(moves: String) -> i32 {
        let (mut dir, mut extra) = (0i32, 0);
        moves.chars().for_each(|chr| match chr {
            'L' => dir -= 1,
            'R' => dir += 1,
            '_' => extra += 1,
            _ => {}
        });

        dir.abs() + extra
    }
}

/// 2839 Check if Strings Can be Made Equal With Operations I
struct Sol2839 {}

impl Sol2839 {
    pub fn can_be_equal(s1: String, s2: String) -> bool {
        for start in [0, 1] {
            let mut s1: Vec<_> = s1.chars().skip(start).step_by(2).collect();
            s1.sort();

            let mut s2: Vec<_> = s2.chars().skip(start).step_by(2).collect();
            s2.sort();

            for (chr1, chr2) in s1.iter().zip(s2.iter()) {
                if chr1 != chr2 {
                    return false;
                }
            }
        }

        true
    }
}

/// 3090 Maximum Length Substring With Two Occurences
struct Sol3090 {}

impl Sol3090 {
    pub fn maximum_length_substring(s: String) -> i32 {
        use std::collections::HashMap;

        {
            let s: Vec<_> = s.chars().collect();
            let mut freqs = HashMap::new();
            let mut l = 0;
            println!(
                ":? {}",
                s.iter().enumerate().fold(0, |longest, (r, &chr)| {
                    freqs.entry(chr).and_modify(|f| *f += 1).or_insert(1);

                    while freqs.get(&chr).unwrap() > &2 {
                        freqs.entry(s[l]).and_modify(|f| *f -= 1);
                        l += 1;
                    }

                    longest.max(r - l + 1)
                })
            );
        }

        (0..s.len())
            .flat_map(|l| (l..s.len()).map(move |r| (l, r)))
            .fold(0, |longest, (l, r)| {
                let mut freqs = HashMap::new();
                for chr in s[l..=r].chars() {
                    freqs.entry(chr).and_modify(|f| *f += 1).or_insert(1);
                }

                if freqs.values().all(|&f| f <= 2) && (r - l + 1) > longest {
                    r - l + 1
                } else {
                    longest
                }
            }) as _
    }
}

/// 3114 Latest Time You Can Obtain After Replacing Characters
struct Sol3114 {}

impl Sol3114 {
    pub fn find_latest_time(s: String) -> String {
        println!(
            ":? {:?}",
            s.chars()
                .enumerate()
                .map(|(i, chr)| match (i, chr) {
                    (0, '?') =>
                        if s.chars().nth(1) <= Some('1') || s.chars().nth(1) == Some('?') {
                            '1'
                        } else {
                            '0'
                        },
                    (1, '?') =>
                        if s.chars().nth(0) == Some('1') || s.chars().nth(0) == Some('?') {
                            '1'
                        } else {
                            '9'
                        },
                    (3, '?') => '5',
                    (4, '?') => '9',
                    (_, chr) => chr,
                })
                .collect::<String>()
        );

        if let [s0, s1, _, s3, s4, ..] = s.chars().collect::<Vec<_>>()[..] {
            for h1 in ['1', '0'] {
                for h2 in ('0'..='9').rev() {
                    if h1 == '1' && h2 > '1' {
                        continue;
                    }

                    for m1 in ('0'..='5').rev() {
                        for m2 in ('0'..='9').rev() {
                            if (s0 == h1 || s0 == '?')
                                && (s1 == h2 || s1 == '?')
                                && (s3 == m1 || s3 == '?')
                                && (s4 == m2 || s4 == '?')
                            {
                                return [h1, h2, ':', m1, m2].iter().collect();
                            }
                        }
                    }
                }
            }
        }

        "00:00".to_string()
    }
}

/// 3280 Convert Date to Binary
struct Sol3280 {}

impl Sol3280 {
    pub fn convert_date_to_binary(date: String) -> String {
        date.split('-')
            .filter_map(|s| s.parse::<i32>().ok())
            .map(|n| format!("{n:b}"))
            .collect::<Vec<_>>()
            .join("-")
    }
}

/// 3340 Check Balanced String
struct Sol3340 {}

impl Sol3340 {
    pub fn is_balanced(num: String) -> bool {
        num.chars()
            .step_by(2)
            .filter_map(|chr| chr.to_digit(10))
            .sum::<u32>()
            == num
                .chars()
                .skip(1)
                .step_by(2)
                .filter_map(|chr| chr.to_digit(10))
                .sum::<u32>()
    }
}

/// 3345 Smallest Divisible Digit Product I
struct Sol3345 {}

impl Sol3345 {
    pub fn smallest_number(mut n: i32, t: i32) -> i32 {
        while n
            .to_string()
            .chars()
            .filter_map(|chr| chr.to_digit(10))
            .product::<u32>()
            % t as u32
            != 0
        {
            n += 1;
        }

        n
    }
}

/// 3438 Find Valid Pair of Adjacent Digits in String
struct Sol3438 {}

impl Sol3438 {
    pub fn find_valid_pair(s: String) -> String {
        use std::collections::HashMap;

        let mut freqs = HashMap::new();
        for chr in s.chars() {
            freqs.entry(chr).and_modify(|f| *f += 1).or_insert(1);
        }
        println!("-> {freqs:?}");

        s.chars()
            .zip(s.chars().skip(1))
            .filter(|(chr, nchr)| {
                chr != nchr
                    && freqs[&chr] == chr.to_digit(10).unwrap()
                    && freqs[&nchr] == nchr.to_digit(10).unwrap()
            })
            .take(1)
            .flat_map(|(chr1, chr2)| vec![chr1, chr2])
            .collect()
    }
}

/// 3602 Hexadeciaml and Hexatrigesimal Conversion
struct Sol3602 {}

impl Sol3602 {
    /// 1 <= n <= 1000
    pub fn concat_hex36(n: i32) -> String {
        let mut chars = vec![];
        for (n, radix) in [(n * n * n, 36), (n * n, 16)] {
            chars.reverse();

            let mut n = n as u32;
            while n > 0 {
                chars.push(char::from_digit(n % radix, radix));
                n /= radix;
            }
            chars.reverse();
        }

        println!("-> {chars:?}");

        chars
            .into_iter()
            .filter_map(|o| o)
            .collect::<String>()
            .to_uppercase()
    }
}

#[cfg(test)]
mod tests;
