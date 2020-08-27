//! # Math

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

/// 2413 Smallest Even Multiple
struct Sol2413 {}

impl Sol2413 {
    pub fn smallest_even_multiple(n: i32) -> i32 {
        println!(":? {}", n << (n & 1));
        if n & 1 == 0 { n } else { 2 * n }
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

#[cfg(test)]
mod tests;
