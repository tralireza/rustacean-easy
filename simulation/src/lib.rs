//! # Simulation

/// 1243 Array Transformation
struct Sol1243 {}

impl Sol1243 {
    pub fn transform_array(mut arr: Vec<i32>) -> Vec<i32> {
        loop {
            let mut n_arr = vec![];
            n_arr.push(*arr.first().unwrap());
            for n in arr.windows(3).map(|w| {
                if w[0] < w[1] && w[2] < w[1] {
                    w[1] - 1
                } else if w[0] > w[1] && w[2] > w[1] {
                    w[1] + 1
                } else {
                    w[1]
                }
            }) {
                n_arr.push(n);
            }
            n_arr.push(*arr.last().unwrap());

            if arr == n_arr {
                return arr;
            }

            println!("-> {n_arr:?}");
            arr = n_arr;
        }
    }
}

/// 3354 Make Array Element Equal to Zero
struct Sol3354 {}

impl Sol3354 {
    pub fn count_valid_selections(nums: Vec<i32>) -> i32 {
        let (mut pfx, mut sfx) = (0, nums.iter().sum::<i32>());
        println!(
            ":? {}",
            nums.iter().fold(0, |mut count, &n| {
                if n == 0 {
                    if pfx == sfx {
                        count += 2;
                    } else if (pfx - sfx).abs() == 1 {
                        count += 1;
                    }
                }
                pfx += n;
                sfx -= n;

                count
            })
        );

        let prefix_sum: Vec<_> = nums
            .iter()
            .scan(0, |csum, n| {
                *csum += n;
                Some(*csum)
            })
            .collect();

        let mut suffix_sum: Vec<_> = nums
            .iter()
            .rev()
            .scan(0, |csum, n| {
                *csum += n;
                Some(*csum)
            })
            .collect();
        suffix_sum.reverse();

        (0..nums.len())
            .filter(|&i| nums[i] == 0)
            .map(|i| {
                if prefix_sum[i] == suffix_sum[i] {
                    2
                } else if (prefix_sum[i] - suffix_sum[i]).abs() == 1 {
                    1
                } else {
                    0
                }
            })
            .sum()
    }
}

/// 3360 Stone Removal Game
struct Sol3360 {}

impl Sol3360 {
    pub fn can_alice_win(mut n: i32) -> bool {
        for (take, &player) in (1..=10).rev().zip(['a', 'b'].iter().cycle()) {
            match player {
                'a' if n < take => return false,
                'b' if n < take => return true,
                _ => n -= take,
            }
        }

        unreachable!()
    }
}

/// 3379 Transformed Array
struct Sol3379 {}

impl Sol3379 {
    pub fn construct_transformed_array(nums: Vec<i32>) -> Vec<i32> {
        let m = nums.len();

        println!(
            ":? {:?}",
            nums.iter()
                .enumerate()
                .map(|(i, &n)| match n {
                    0.. => (i + n as usize) % m,
                    ..0 => (i as i32 + n % m as i32 + m as i32) as usize % m,
                })
                .map(|i| nums[i])
                .collect::<Vec<_>>()
        );

        nums.iter()
            .enumerate()
            .map(|(i, &n)| (i as i32 + n % m as i32 + m as i32) as usize % m)
            .map(|i| nums[i])
            .collect()
    }
}

#[cfg(test)]
mod tests;
