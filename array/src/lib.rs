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

#[cfg(test)]
mod tests;
