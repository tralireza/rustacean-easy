//! # Array

/// 2389 Longest Subsequence With Limited Sum
struct Sol2389 {}

impl Sol2389 {
    pub fn answer_queries(mut nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        nums.sort();

        let mut answers = vec![];
        for x in queries {
            let mut dsum = 0;
            answers.push(
                nums.iter()
                    .take_while(|&n| {
                        dsum += n;
                        dsum <= x
                    })
                    .count() as i32,
            );
        }

        answers
    }
}

/// 2404 Most Frequest Even Element
struct Sol2404 {}

impl Sol2404 {
    pub fn most_frequent_even(nums: Vec<i32>) -> i32 {
        use std::collections::BTreeMap;

        let mut freqs = BTreeMap::new();
        nums.iter().filter(|&n| n & 1 == 0).for_each(|n| {
            freqs.entry(n).and_modify(|f| *f += 1).or_insert(1);
        });

        if let Some(xfreq) = freqs.values().max() {
            freqs
                .iter()
                .find(|&(_, f)| f == xfreq)
                .map(|(&&n, _)| n)
                .unwrap()
        } else {
            -1
        }
    }
}

/// 2432 The Employee That Worked on the Longest Task
struct Sol2432 {}

impl Sol2432 {
    pub fn hardest_worker(n: i32, logs: Vec<Vec<i32>>) -> i32 {
        println!("-> {n}");

        logs.windows(2)
            .fold((logs[0][0], logs[0][1]), |(wkr, ltask), w| {
                let l = w[1][1] - w[0][1];
                if l > ltask || l == ltask && w[1][0] < wkr {
                    (w[1][0], l)
                } else {
                    (wkr, ltask)
                }
            })
            .0
    }
}

/// 2614 Prime In Diagonal
struct Sol2614 {}

impl Sol2614 {
    pub fn diagonal_prime(nums: Vec<Vec<i32>>) -> i32 {
        *nums
            .iter()
            .enumerate()
            .filter_map(|(d, row)| {
                row.iter()
                    .skip(d)
                    .take(1)
                    .zip(row.iter().rev().skip(d).take(1))
                    .next()
            })
            .map(|(x, y)| vec![x, y])
            .flatten()
            .filter(|&&n| n > 1)
            .filter(|&&n| {
                let mut m = 2;
                while m * m <= n {
                    if n % m == 0 {
                        return false;
                    }
                    m += 1;
                }

                true
            })
            .max()
            .unwrap_or(&0)
    }
}

#[cfg(test)]
mod tests;
