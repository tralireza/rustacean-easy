//! # DP (aka: Dynamic Programming)

/// 198m House Robber
struct Sol198 {}

impl Sol198 {
    pub fn rob(nums: Vec<i32>) -> i32 {
        use std::collections::{BTreeMap, HashMap};

        let mut cache = HashMap::from([(0, nums[0]), (1, nums[0].max(nums[1]))]);
        let mut sorted_cache = BTreeMap::from([(0, nums[0]), (1, nums[0].max(nums[1]))]);

        fn search(
            i: usize,
            nums: &[i32],
            cache: &mut HashMap<usize, i32>,
            sorted_cache: &mut BTreeMap<usize, i32>,
        ) -> i32 {
            if cache.contains_key(&i) {
                return cache[&i];
            }

            let value = (search(i - 2, nums, cache, sorted_cache) + nums[i]).max(search(
                i - 1,
                nums,
                cache,
                sorted_cache,
            ));

            cache.insert(i, value);
            sorted_cache.insert(i, value);

            println!("-> {sorted_cache:?} | {cache:?}");

            value
        }

        search(nums.len() - 1, &nums, &mut cache, &mut sorted_cache)
    }
}

#[cfg(test)]
mod tests;
