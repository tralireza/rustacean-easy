//! # Sliding Window

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
            .inspect(|e| println!("-> {e:?}"))
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
                    .inspect(|state| println!("   {state:?}"))
                    .map(|(_, _, _, longest)| longest + 1)
                    .max()
                    .unwrap_or(0)
                    .max(longest)
            }) as _
    }
}

#[cfg(test)]
mod tests;
