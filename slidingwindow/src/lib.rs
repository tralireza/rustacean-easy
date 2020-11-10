//! # Sliding Window

/// 1176 Diet Plan Performance
struct Sol1176 {}

impl Sol1176 {
    /// 1 <= K <= N <= 10^5
    /// 0 <= C_i <= 2000
    pub fn diet_plan_performance(calories: Vec<i32>, k: i32, lower: i32, upper: i32) -> i32 {
        {
            let mut cur_sum = calories.iter().take(k as usize - 1).sum::<i32>();
            println!(
                ":? {}",
                calories
                    .iter()
                    .skip(k as usize - 1)
                    .scan(0, |prv, calory| {
                        cur_sum += calory;

                        let mut point = 0;
                        if cur_sum < lower {
                            point -= 1;
                        } else if cur_sum > upper {
                            point += 1;
                        }

                        cur_sum -= calories[*prv];
                        *prv += 1;

                        Some(point)
                    })
                    .sum::<i32>()
            );
        }

        calories
            .windows(k as usize)
            .map(|w| w.iter().sum::<i32>())
            .map(|total| {
                if total < lower {
                    -1
                } else if total > upper {
                    1
                } else {
                    0
                }
            })
            .sum()
    }
}

/// 1493m Longest Subarray of 1's After Deleting One Element
struct Sol1493 {}

impl Sol1493 {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut zeros = 0;
        let mut left = 0;

        nums.iter().enumerate().fold(0, |longest, (right, &n)| {
            if n == 0 {
                zeros += 1;
            }

            while zeros > 1 {
                if nums[left] == 0 {
                    zeros -= 1;
                }
                left += 1;
            }

            longest.max(right - left)
        }) as _
    }
}

/// 3364 Minimum Positive Sum Subarray
struct Sol3364 {}

impl Sol3364 {
    pub fn minimum_sum_subarray(nums: Vec<i32>, l: i32, r: i32) -> i32 {
        let mut msum = i32::MAX;

        for w in l..=r {
            msum = nums.windows(w as usize).fold(msum, |msum, sarr| {
                let sum = sarr.iter().sum();
                if sum > 0 { msum.min(sum) } else { msum }
            });
        }

        if msum < i32::MAX {
            return msum;
        }
        -1
    }
}

/// 3411 Maximum Subarray With Equal Products
struct Sol3411 {}

impl Sol3411 {
    pub fn max_length(nums: Vec<i32>) -> i32 {
        fn gcd(mut a: i32, mut b: i32) -> i32 {
            while b != 0 {
                (a, b) = (b, a % b);
            }
            a
        }

        nums.iter()
            .take(nums.len() - 1)
            .enumerate()
            .inspect(|(_, n)| println!("-> {n:?}"))
            .fold(1, |longest, (i, &n)| {
                nums.iter()
                    .skip(i + 1)
                    .enumerate()
                    .scan((n, n, n, 0), |(prd, g, l, _), (i, &n)| {
                        *prd *= n;
                        *g = gcd(*g, n);
                        *l *= n / gcd(*l, n);

                        if *prd == *g * *l {
                            Some((*prd, *g, *l, i + 1))
                        } else {
                            None
                        }
                    })
                    .inspect(|state| println!(" {state:?}"))
                    .map(|(_, _, _, longest)| longest + 1)
                    .max()
                    .unwrap_or(1)
                    .max(longest)
            }) as _
    }
}

#[cfg(test)]
mod tests;
