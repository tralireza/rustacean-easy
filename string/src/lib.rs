//! # String

/// 245 Shortest Word Distance
struct Sol245 {}

impl Sol245 {
    pub fn shortest_distance(words_dict: Vec<String>, word1: String, word2: String) -> i32 {
        words_dict
            .iter()
            .enumerate()
            .filter(|(_, w)| **w == word1)
            .map(|(i, _)| {
                words_dict
                    .iter()
                    .enumerate()
                    .filter(|(_, w)| **w == word2)
                    .map(|(j, _)| (i as i32 - j as i32).abs())
                    .min()
                    .unwrap()
            })
            .min()
            .unwrap() as _
    }
}

/// 266 Palindrome Permutation
struct Sol266 {}

impl Sol266 {
    pub fn can_permute_palindrome(s: String) -> bool {
        use std::collections::HashMap;

        let mut freqs = HashMap::new();
        for chr in s.chars() {
            freqs.entry(chr).and_modify(|f| *f += 1).or_insert(1);
        }

        freqs.values().filter(|&f| f & 1 == 1).count() <= 1
    }
}

/// 293 Flip Game
struct Sol293 {}

impl Sol293 {
    pub fn generate_possible_next_moves(current_state: String) -> Vec<String> {
        (0..current_state.len() - 1)
            .filter(|&i| &current_state[i..i + 2] == "++")
            .map(|i| current_state[..i].to_string() + "--" + &current_state[i + 2..])
            .collect()
    }
}

/// 800 Similar RGB Color
struct Sol800 {}

impl Sol800 {
    pub fn similar_rgb(color: String) -> String {
        {
            println!(
                ":? {:?}",
                (1..color.len())
                    .step_by(2)
                    .flat_map(|i| i32::from_str_radix(&color[i..i + 2], 16))
                    .fold("#".to_string(), |similar, x| {
                        similar
                            + &format!(
                                "{:x}{:x}",
                                (x + (17 >> 1)) / 17, // .round()
                                (x as f64 / 17.0).round() as i32
                            )
                    })
            );
        }

        let color: Vec<_> = color.chars().skip(1).collect();
        color
            .chunks(2)
            .map(|chk| 16 * chk[0].to_digit(16).unwrap() + chk[1].to_digit(16).unwrap())
            .map(|x| {
                format!(
                    "{:x}{:x}",
                    (x as f64 / 17.0).round() as u32,
                    (x as f64 / 17.0).round() as u32
                )
            })
            .fold("#".to_string(), |similar, s| similar + &s)
    }
}

/// 1065 Index Pair of a String
struct Sol1065 {}

impl Sol1065 {
    pub fn index_pairs(text: String, words: Vec<String>) -> Vec<Vec<i32>> {
        {
            use std::collections::HashMap;

            #[derive(Default, Debug)]
            struct Trie {
                children: HashMap<char, Trie>,
                is_word: bool,
            }

            let mut trie = Trie::default();

            for word in &words {
                let mut n = &mut trie;
                for chr in word.chars() {
                    n = n.children.entry(chr).or_default();
                }
                n.is_word = true;
            }
            println!("-> Trie: {trie:?}");

            let text: Vec<_> = text.chars().collect();
            let mut pairs: Vec<_> = vec![];
            for l in 0..text.len() {
                let mut n = &trie;

                for (r, chr) in text.iter().enumerate().skip(l) {
                    if let Some(c) = n.children.get(chr) {
                        n = c;
                        if n.is_word {
                            pairs.push(vec![l, r]);
                        }
                    } else {
                        break;
                    }
                }
            }
            println!(":? {:?}", pairs);
        }

        (0..text.len())
            .flat_map(|l| (l..text.len()).map(move |r| (l, r)))
            .fold(vec![], |mut pairs, (l, r)| {
                if words.contains(&text[l..=r].to_string()) {
                    pairs.push(vec![l as i32, r as i32]);
                }
                pairs
            })
    }
}

/// 1180 Count Substrings with Only One Distinct Letter
struct Sol1180 {}

impl Sol1180 {
    pub fn count_letters(s: String) -> i32 {
        s.chars()
            .collect::<Vec<_>>()
            .chunk_by(|chr, n_chr| chr == n_chr)
            .map(|chunk| (chunk.len() + 1) * chunk.len() / 2)
            .sum::<usize>() as _
    }
}

/// Hexspeak
struct Sol1271 {}

impl Sol1271 {
    pub fn to_hexspeak(num: String) -> String {
        if let Ok(num) = num.parse::<i64>() {
            println!("{num}");
            let hexspeak: String = format!("{:x}", num)
                .chars()
                .map(|chr| match chr {
                    '1' => 'I',
                    '0' => 'O',
                    _ => chr.to_ascii_uppercase(),
                })
                .collect();
            if hexspeak.chars().all(|chr| "ABCDEFIO".contains(chr)) {
                hexspeak
            } else {
                "ERROR".to_string()
            }
        } else {
            "ERROR".to_string()
        }
    }
}

/// 1427 Perform String Shifts
struct Sol1427 {}

impl Sol1427 {
    pub fn string_shift(s: String, shift: Vec<Vec<i32>>) -> String {
        let shifts = shift
            .iter()
            .map(|v| ((v[0] << 1) - 1) * v[1] % s.len() as i32)
            .map(|shift| {
                (if shift < 0 {
                    shift + s.len() as i32
                } else {
                    shift
                }) as usize
            })
            .sum::<usize>()
            % s.len();

        println!("-> {shifts}");

        s.chars().skip(s.len() - shifts).collect::<String>()
            + &s.chars().take(s.len() - shifts).collect::<String>()
    }
}

/// 1933 Check if String Is Decomposable Into Value-Equal Substrings
struct Sol1933 {}

impl Sol1933 {
    pub fn is_decomposable(s: String) -> bool {
        let mut prv = '*';

        let mut counter = 0;
        let mut has_two = false;
        for chr in s.chars() {
            if counter == 0 {
                counter += 1;
                prv = chr;
            } else if prv == chr {
                counter += 1;
                if counter == 3 {
                    counter = 0;
                }
            } else if counter == 2 {
                if has_two {
                    return false;
                }
                has_two = true;
                counter = 1;
                prv = chr;
            } else {
                return false;
            }
        }

        counter != 1 && (has_two || counter == 2)
    }
}

/// 1935 Maximum Number of Words You Can Type
struct Sol1935 {}

impl Sol1935 {
    pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
        text.split(' ')
            .filter(|w| !broken_letters.chars().any(|chr| w.contains(chr)))
            .count() as _
    }
}

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

/// 3362 Filter Characters by Frequency
struct Sol3362 {}

impl Sol3362 {
    pub fn filter_characters(s: String, k: i32) -> String {
        use std::collections::HashMap;

        let mut freqs = HashMap::new();
        for chr in s.chars() {
            freqs.entry(chr).and_modify(|f| *f += 1).or_insert(1);
        }

        s.chars().filter(|chr| freqs[chr] < k).collect()
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

/// 3541 Find Most Frequent Vowel and Consonant
struct Sol3541 {}

impl Sol3541 {
    pub fn max_freq_sum(s: String) -> i32 {
        use std::collections::HashMap;

        let freqs: HashMap<char, i32> = s.chars().fold(HashMap::new(), |mut freqs, chr| {
            freqs.entry(chr).and_modify(|f| *f += 1).or_insert(1);
            freqs
        });

        println!("-> {freqs:?}");

        freqs
            .iter()
            .filter(|(chr, _)| "aeiou".contains(**chr))
            .map(|(_, &f)| f)
            .max()
            .unwrap_or(0)
            + freqs
                .iter()
                .filter(|(chr, _)| !"aeiou".contains(**chr))
                .map(|(_, &f)| f)
                .max()
                .unwrap_or(0)
    }
}

/// 3571 Find the Shortest Superstring II
struct Sol3571 {}

impl Sol3571 {
    pub fn shortest_superstring(s1: String, s2: String) -> String {
        fn overlap<'a>(s1: &'a str, s2: &str) -> &'a str {
            for start in 0..s1.len() {
                if s1[start..]
                    .chars()
                    .zip(s2.chars())
                    .all(|(chr1, chr2)| chr1 == chr2)
                {
                    return &s1[start..];
                }
            }

            ""
        }

        let cn12 = overlap(&s1[..], &s2[..]);
        let cn21 = overlap(&s2[..], &s1[..]);

        if cn12.len() >= cn21.len() {
            s1.clone() + &s2[cn12.len().min(s2.len())..]
        } else {
            s2.clone() + &s1[cn21.len().min(s1.len())..]
        }
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
            .flatten()
            .collect::<String>()
            .to_uppercase()
    }
}

/// 3606 Coupon Code Validator
struct Sol3606 {}

impl Sol3606 {
    pub fn validate_coupons(
        code: Vec<String>,
        business_line: Vec<String>,
        is_active: Vec<bool>,
    ) -> Vec<String> {
        use std::iter::zip;

        let mut valids: Vec<_> = zip(business_line, code)
            .zip(is_active)
            .filter_map(|(bc, a)| a.then_some(bc))
            .filter(|(b, _)| b != "invalid")
            .filter(|(_, c)| {
                !c.is_empty()
                    && c.chars().all(|chr| {
                        chr.is_lowercase() || chr.is_uppercase() || chr.is_digit(10) || chr == '_'
                    })
            })
            .collect();

        println!("-> {valids:?}");
        valids.sort();

        valids.into_iter().map(|(_, c)| c).collect()
    }
}

#[cfg(test)]
mod tests;
