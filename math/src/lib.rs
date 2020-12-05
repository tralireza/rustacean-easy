//! # Math

/// 812 Largest Triangle Area
struct Sol812 {}

impl Sol812 {
    /// 3 <= N <= 50
    pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
        fn gauss_area(p: &[i32], q: &[i32], r: &[i32]) -> i32 {
            (p[0] * q[1] - p[1] * q[0] + q[0] * r[1] - q[1] * r[0] + r[0] * p[1] - r[1] * p[0])
                .abs()
        }

        points
            .iter()
            .enumerate()
            .take(points.len() - 2)
            .flat_map(|(i, p)| {
                points
                    .iter()
                    .enumerate()
                    .take(points.len() - 1)
                    .skip(i + 1)
                    .flat_map(|(j, q)| points.iter().skip(j + 1).map(|r| gauss_area(p, q, r)))
            })
            .max()
            .unwrap() as f64
            * 0.5
    }
}

/// 1056 Confusing Number
struct Sol1056 {}

impl Sol1056 {
    pub fn confusing_number(n: i32) -> bool {
        n.to_string().chars().all(|chr| "01689".contains(chr))
            && n.to_string()
                .chars()
                .map(|chr| match chr {
                    '6' => '9',
                    '9' => '6',
                    _ => chr,
                })
                .rev()
                .collect::<String>()
                != n.to_string()
    }
}

/// 1085 Sum of Digits in the Minimum Number
struct Sol1085 {}

impl Sol1085 {
    pub fn sum_of_digits(nums: Vec<i32>) -> i32 {
        match nums.into_iter().min() {
            Some(mut vmin) => {
                let mut dsum = 0;
                while vmin > 0 {
                    dsum += vmin % 10;
                    vmin /= 10;
                }
                1 ^ (dsum & 1)
            }
            _ => -1,
        }
    }
}

/// 1118 Number of Days in a Month
struct Sol1118 {}

impl Sol1118 {
    pub fn number_of_days(year: i32, month: i32) -> i32 {
        let mut days = [-1, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        if year % 400 == 0 || year % 4 == 0 && year % 100 != 0 {
            days[2] += 1;
        }

        days[month as usize]
    }
}

/// 1182m Shortest Distance to Target Color
struct Sol1182 {}

impl Sol1182 {
    /// 1 <= Colors, Queries <= 5*10^4
    pub fn shortest_distance_color(colors: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        {
            let mut dists = vec![vec![]; 3];
            for (c, c_dists) in dists.iter_mut().enumerate() {
                *c_dists = colors
                    .iter()
                    .enumerate()
                    .filter(|&(_, &color)| color as usize - 1 == c)
                    .map(|(i, _)| i)
                    .collect();
            }
            println!("-> {dists:?}");

            fn lbsearch(sarr: &[usize], t: usize) -> usize {
                let (mut l, mut r) = (0, sarr.len());
                while l < r {
                    let m = l + ((r - l) >> 1);
                    if sarr[m] < t {
                        l = m + 1;
                    } else {
                        r = m;
                    }
                }

                l
            }

            println!(
                ":? {:?}",
                queries
                    .iter()
                    .map(|qry| (qry[0] as usize, qry[1] as usize))
                    .map(|(index, color)| {
                        let cds = &dists[color - 1];

                        if cds.is_empty() {
                            -1
                        } else {
                            let x = lbsearch(cds, index);
                            (if x == 0 {
                                cds[x] - index
                            } else if x == cds.len() {
                                index - cds[x - 1]
                            } else {
                                (index - cds[x]).min(cds[x + 1] - index)
                            }) as i32
                        }
                    })
                    .collect::<Vec<_>>()
            );
        }

        let mut dists = vec![vec![i32::MAX; 3]; colors.len() + 2];

        for (i, &color) in colors.iter().enumerate() {
            for c in 0..3 {
                dists[i + 1][c] = if c == color as usize - 1 {
                    0
                } else {
                    dists[i][c].saturating_add(1)
                };
            }
        }
        println!("-> >>> {dists:?}");

        for (i, &color) in colors.iter().enumerate().rev() {
            for c in 0..3 {
                dists[i + 1][c] = if c == color as usize - 1 {
                    0
                } else {
                    dists[i + 1][c].min(dists[i + 2][c].saturating_add(1))
                };
            }
        }
        println!("-> <<< {dists:?}");

        queries
            .iter()
            .map(|qry| (qry[0] as usize, qry[1] as usize))
            .map(|(index, color)| dists[index + 1][color - 1])
            .map(|distance| if distance == i32::MAX { -1 } else { distance })
            .collect()
    }
}

/// 1228 Missing Number In Arithmetic Progression
struct Sol1228 {}

impl Sol1228 {
    pub fn missing_number(arr: Vec<i32>) -> i32 {
        let d = (arr.last().unwrap() - arr.first().unwrap()) / arr.len() as i32;
        println!("-> Arithmetic Difference (d): {d}");

        {
            let (mut l, mut r) = (0, arr.len() - 1);
            while l < r {
                let m = l + ((r - l) >> 1);
                if arr[m] == arr[0] + d * m as i32 {
                    l = m + 1;
                } else {
                    r = m;
                }
            }

            println!(":? O(log(N)) Binary Search : {}", arr[0] + l as i32 * d);
        }

        for w in arr.windows(2) {
            if w[1] - w[0] != d {
                return w[0] + d;
            }
        }

        arr[0]
    }
}

/// 1317 Convert Integer to the Sum of Two No-Zero Integers
struct Sol1317 {}

impl Sol1317 {
    pub fn get_no_zero_integers(n: i32) -> Vec<i32> {
        {
            let mut a = 1;
            println!(
                ":? {:?}",
                loop {
                    if !a.to_string().contains('0') && !(n - a).to_string().contains('0') {
                        break vec![a, n - a];
                    }
                    a += 1;
                }
            );
        }

        let no_zero = |mut n| {
            while n > 0 {
                if n % 10 == 0 {
                    return false;
                }
                n /= 10;
            }
            true
        };

        let mut a = 1;
        loop {
            if no_zero(a) && no_zero(n - a) {
                return vec![a, n - a];
            }
            a += 1;
        }
    }
}

/// 1323 Maximum 69 Number
struct Sol1323 {}

impl Sol1323 {
    pub fn maximum69_number(mut num: i32) -> i32 {
        println!(
            ":? {:?}",
            num.to_string()
                .replacen("6", "9", 1)
                .parse::<i32>()
                .unwrap()
        );

        let mut digits = vec![];
        while num > 0 {
            digits.push(num % 10);
            num /= 10;
        }
        digits.reverse();

        let mut swap = true;
        digits
            .iter()
            .map(|&d| {
                if d == 6 && swap {
                    swap = false;
                    9
                } else {
                    d
                }
            })
            .fold(0, |x69, d| 10 * x69 + d)
    }
}

/// 1689m Partitioning Into Minimum Number Of Deci-Binary Numbers
struct Sol1689 {}

impl Sol1689 {
    pub fn min_partitions(n: String) -> i32 {
        n.chars().flat_map(|chr| chr.to_digit(10)).max().unwrap() as _
    }
}

/// 2348m Number of Zero-Filled Subarrays
struct Sol2348 {}

impl Sol2348 {
    pub fn zero_filled_subarray(nums: Vec<i32>) -> i64 {
        nums.iter()
            .fold((0, 0), |(count, mut zeros), &n| {
                if n == 0 {
                    zeros += 1;
                } else {
                    zeros = 0;
                }

                (count + zeros, zeros)
            })
            .0
    }
}

/// 2396m Strictly Palindromic Number
struct Sol2396 {}

impl Sol2396 {
    pub fn is_strictly_palindromic(n: i32) -> bool {
        println!(":? false");

        (2..=n - 2).all(|base| {
            let mut digits = vec![];
            let mut n = n;
            while n > 0 {
                digits.push(n % base);
                n /= base;
            }

            digits
                .iter()
                .zip(digits.iter().rev())
                .take(digits.len() / 2)
                .all(|(d, q)| d == q)
        })
    }
}

/// 2413 Smallest Even Multiple
struct Sol2413 {}

impl Sol2413 {
    pub fn smallest_even_multiple(n: i32) -> i32 {
        println!(":? {}", n << (n & 1));
        if n & 1 == 0 { n } else { 2 * n }
    }
}

/// 2433m Find The Original Array of Prefix Xor
struct Sol2433 {}

impl Sol2433 {
    pub fn find_array(pref: Vec<i32>) -> Vec<i32> {
        println!(
            ":? {:?}",
            pref[..1]
                .iter()
                .copied()
                .chain(pref.windows(2).map(|w| w[0] ^ w[1]))
                .collect::<Vec<_>>()
        );

        pref.iter()
            .take(1)
            .copied()
            .chain(
                pref.iter()
                    .zip(pref.iter().skip(1))
                    .map(|(&cur, &next)| cur ^ next),
            )
            .collect()
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

/// 2544 Alternate Digit Sum
struct Sol2544 {}

impl Sol2544 {
    pub fn alternate_digit_sum(mut n: i32) -> i32 {
        let mut ds = vec![];
        while n > 0 {
            ds.push(n % 10);
            n /= 10;
        }
        ds.reverse();

        println!(
            ":? {}",
            ds.iter()
                .zip([1, -1].iter().cycle())
                .map(|(d, sign)| d * sign)
                .sum::<i32>()
        );

        ds.iter()
            .zip([1, -1].iter().cycle())
            .fold(0, |d_sum, (d, sign)| d_sum + sign * d)
    }
}

/// 2578 Split With Minimum Sum
struct Sol2578 {}

impl Sol2578 {
    pub fn split_num(mut num: i32) -> i32 {
        let mut digits = vec![];
        while num > 0 {
            digits.push(num % 10);
            num /= 10;
        }
        digits.sort();

        digits
            .iter()
            .enumerate()
            .filter(|(i, _)| i & 1 == 0)
            .fold(0, |n, (_, &d)| 10 * n + d)
            + digits
                .iter()
                .enumerate()
                .filter(|(i, _)| i & 1 == 1)
                .fold(0, |n, (_, &d)| 10 * n + d)
    }
}

/// 2928 Distribute Candies Among Children I
struct Sol2928 {}

impl Sol2928 {
    pub fn distribute_candies(n: i32, limit: i32) -> i32 {
        let mut ways = 0;
        for first in 0..=n.min(limit) {
            for second in 0..=(n - first).min(limit) {
                println!("-> {first} {second} ? {}", n - (first + second));

                if 0 <= n - (first + second) && n - (first + second) <= limit {
                    ways += 1;
                }
            }
        }

        ways
    }
}

/// 2932 Maximum Strong Pair XOR I
struct Sol2932 {}

impl Sol2932 {
    pub fn maximum_strong_pair_xor(nums: Vec<i32>) -> i32 {
        nums.iter().enumerate().fold(0, |x_xor, (i, &n)| {
            nums.iter()
                .skip(i)
                .filter(|&&p| (p - n).abs() <= p.min(n))
                .map(|&p| p ^ n)
                .max()
                .unwrap_or(0)
                .max(x_xor)
        })
    }
}

/// 3199 Count Triplets with Even XOR Set Bits I
struct Sol3199 {}

impl Sol3199 {
    /// 1 <= N (a/b/c) <= 100
    pub fn triplet_count(a: Vec<i32>, b: Vec<i32>, c: Vec<i32>) -> i32 {
        (0..a.len())
            .flat_map(|i| (0..b.len()).map(move |j| (i, j)))
            .flat_map(|(i, j)| (0..c.len()).map(move |k| (i, j, k)))
            .inspect(|t| println!("-> {t:?}"))
            .filter(|&(i, j, k)| (a[i] ^ b[j] ^ c[k]).count_ones() & 1 == 0)
            .count() as _
    }
}

/// 3658 GCD of Odd and Even Sums
struct Sol3658 {}

impl Sol3658 {
    pub fn gcd_of_odd_even_sums(n: i32) -> i32 {
        println!(":? {n}");

        let mut b = (1..).step_by(2).take(n as usize).sum();
        let mut a = b + n;

        while b > 0 {
            (a, b) = (b, a % b);
        }

        a
    }
}

/// 3674 Minimum Operations to Equalize Array
struct Sol3674 {}

impl Sol3674 {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        if nums.iter().all(|n| n == nums.first().unwrap()) {
            return 0;
        }

        1
    }
}

#[cfg(test)]
mod tests;
