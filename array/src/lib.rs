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

#[cfg(test)]
mod tests;
