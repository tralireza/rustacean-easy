//! # Sliding Window

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
