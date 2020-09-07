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

        let mut xvals = vec![0; nums.len()];
        (xvals[0], xvals[1]) = (nums[0], nums[0].max(nums[1]));
        for i in 2..xvals.len() {
            xvals[i] = (xvals[i - 2] + nums[i]).max(xvals[i - 1]);
        }

        println!(":? {:?} {xvals:?}", xvals.last());

        search(nums.len() - 1, &nums, &mut cache, &mut sorted_cache)
    }
}

/// 746 Min Cost Climbing Stairs
struct Sol746 {}

impl Sol746 {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut m_cost = vec![0; cost.len()];
        m_cost[0] = cost[0];
        m_cost[1] = cost[1];

        for i in 2..m_cost.len() {
            m_cost[i] = cost[i] + m_cost[i - 2].min(m_cost[i - 1]);
        }

        println!(
            ":? {} | {m_cost:?}",
            m_cost[m_cost.len() - 2].min(m_cost[m_cost.len() - 1])
        );

        m_cost.into_iter().skip(cost.len() - 2).min().unwrap()
    }
}

#[cfg(test)]
mod tests;
