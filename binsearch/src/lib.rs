//! # Binary Search

/// 1099 Two Sum Less Than K
struct Sol1099 {}

impl Sol1099 {
    pub fn two_sum_less_than_k(mut nums: Vec<i32>, k: i32) -> i32 {
        {
            nums.sort();

            let mut xsum = -1;
            let (mut l, mut r) = (0, nums.len() - 1);
            while l < r {
                let cur_sum = nums[l] + nums[r];
                if cur_sum < k {
                    xsum = xsum.max(cur_sum);
                    l += 1;
                } else {
                    r -= 1;
                }
            }

            println!(":? O(N*log(N)) {xsum}");
        }

        nums.iter()
            .take(nums.len() - 1)
            .enumerate()
            .flat_map(|(i, x)| {
                nums.iter()
                    .skip(i + 1)
                    .filter(|&&y| x + y < k)
                    .map(|&y| y + x)
                    .max()
            })
            .max()
            .unwrap_or(-1)
    }
}

/// 1150 Check if a Number Is Majority Element in a Sorted Array
struct Sol1150 {}

impl Sol1150 {
    pub fn is_majority_element(nums: Vec<i32>, target: i32) -> bool {
        {
            fn lbsearch(arr: &[i32], target: i32) -> usize {
                let (mut l, mut r) = (0, arr.len());
                while l < r {
                    let m = l + ((r - l) >> 1);
                    if arr[m] < target {
                        l = m + 1;
                    } else {
                        r = m;
                    }
                }

                l
            }

            fn rbsearch(arr: &[i32], target: i32) -> usize {
                let (mut l, mut r) = (0, arr.len());
                while l < r {
                    let m = l + ((r - l) >> 1);
                    if arr[m] > target {
                        r = m;
                    } else {
                        l = m + 1;
                    }
                }

                r - 1
            }

            let l = lbsearch(&nums, target);
            println!(
                ":? O(log(N)) {}",
                l < nums.len() / 2 && nums[l + nums.len() / 2] == target
            );

            let r = rbsearch(&nums, target);
            println!(":? O(log(N)) {}", r - l >= nums.len() / 2);
        }

        nums.iter().filter(|&&n| n == target).count() > nums.len() / 2
    }
}

#[cfg(test)]
mod tests;
