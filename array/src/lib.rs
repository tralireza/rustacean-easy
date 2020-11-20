//! # Array

/// 118 Pascal's Triangle
struct Sol118 {}

impl Sol118 {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut triangle = vec![vec![1; 1]];
        for r in 1..num_rows as usize {
            let mut row = vec![1; 1];
            triangle[r - 1]
                .windows(2)
                .for_each(|w| row.push(w.iter().sum::<i32>()));
            row.push(1);

            triangle.push(row);
        }

        triangle
    }
}

/// 163 Missing Ranges
struct Sol163 {}

impl Sol163 {
    /// -10^9 <= L <= U <= 10^9
    pub fn find_missing_ranges(nums: Vec<i32>, lower: i32, upper: i32) -> Vec<Vec<i32>> {
        if nums.is_empty() {
            return vec![vec![lower, upper]];
        }

        let mut missing: Vec<_> = nums
            .windows(2)
            .flat_map(|w| {
                if w[0] + 1 == w[1] {
                    None
                } else {
                    Some(vec![w[0] + 1, w[1] - 1])
                }
            })
            .collect();

        if let Some(&first) = nums.first() {
            if lower < first {
                missing.insert(0, vec![lower, first - 1]);
            }
        }

        if let Some(&last) = nums.last() {
            if last < upper {
                missing.push(vec![last + 1, upper]);
            }
        }

        missing
    }
}

/// 346 Moving Average from Data Stream
struct Sol346 {
    queue: std::collections::VecDeque<i32>,
    win_size: usize,
}

impl Sol346 {
    fn new(size: i32) -> Self {
        Sol346 {
            queue: std::collections::VecDeque::new(),
            win_size: size as usize,
        }
    }

    fn next(&mut self, val: i32) -> f64 {
        if self.queue.len() >= self.win_size {
            self.queue.pop_front();
        }
        self.queue.push_back(val);

        self.queue.iter().sum::<i32>() as f64 / self.queue.len() as f64
    }
}

/// 1064 Fixed Point
struct Sol1064 {}

impl Sol1064 {
    pub fn fixed_point(arr: Vec<i32>) -> i32 {
        {
            let mut x = -1;

            let (mut l, mut r) = (0, arr.len() as i32 - 1);
            while l <= r {
                let m = l + ((r - l) >> 1);
                if arr[m as usize] >= m {
                    r = m - 1;
                    if arr[m as usize] == m {
                        x = m;
                    }
                } else {
                    l = m + 1;
                }
            }

            println!(":? O(log(N)): {x}");
        }

        arr.into_iter()
            .enumerate()
            .filter(|&(i, n)| i as i32 == n)
            .map(|(_, n)| n)
            .nth(0)
            .unwrap_or(-1)
    }
}

/// 1086 High Five
struct Sol1086 {}

impl Sol1086 {
    pub fn high_five(mut items: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        use std::cmp::Reverse;

        items.sort_by_key(|v| (v[0], Reverse(v[1])));
        println!("-> {items:?}");

        items
            .chunk_by(|v, w| v[0] == w[0])
            .map(|chk| vec![chk[0][0], chk.iter().take(5).map(|v| v[1]).sum::<i32>() / 5])
            .collect()
    }
}

/// 1133 Largest Unique Number
struct Sol1133 {}

impl Sol1133 {
    pub fn largest_unique_number(mut nums: Vec<i32>) -> i32 {
        use std::cmp::Reverse;

        nums.sort_by_key(|&n| Reverse(n));
        nums.chunk_by(|x, y| x == y)
            .filter(|chk| chk.len() == 1)
            .nth(0)
            .map(|chk| chk[0])
            .unwrap_or(-1)
    }
}

/// 1304 Find N Unique Integers Sum up to Zero
struct Sol1304 {}

impl Sol1304 {
    pub fn sum_zero(n: i32) -> Vec<i32> {
        let mut szero = vec![];
        if n & 1 == 1 {
            szero.push(0);
        }

        (1..n / 2 as i32).fold(szero, |mut szero, n| {
            szero.push(n);
            szero.push(-n);

            szero
        })
    }
}

/// 1708 Largest Subarray Length K
struct Sol1708 {}

impl Sol1708 {
    pub fn largest_subarray(nums: Vec<i32>, k: i32) -> Vec<i32> {
        nums.windows(k as usize)
            .max_by_key(|w| w[0])
            .unwrap()
            .to_vec()
    }
}

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
            .flat_map(|(x, y)| vec![x, y])
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

/// 2639 Find the Width of Columns of a Grid
struct Sol2639 {}

impl Sol2639 {
    pub fn find_column_width(grid: Vec<Vec<i32>>) -> Vec<i32> {
        (0..grid[0].len())
            .map(|c| {
                grid.iter()
                    .map(|row| row[c].to_string().len() as i32)
                    .max()
                    .unwrap_or(0)
            })
            .collect()
    }
}

/// 2660 Determine the Winnder of a Bowling  Game
struct Sol2660 {}

impl Sol2660 {
    pub fn is_winner(player1: Vec<i32>, player2: Vec<i32>) -> i32 {
        let mut scores = vec![];

        for player in [player1, player2] {
            let mut score = player[0];
            if player.len() > 1 {
                if player[0] == 10 {
                    score += 2 * player[1];
                } else {
                    score += player[1];
                }
            }

            score += player
                .windows(3)
                .map(|w| {
                    if w[0] == 10 || w[1] == 10 {
                        2 * w[2]
                    } else {
                        w[2]
                    }
                })
                .sum::<i32>();
            scores.push(score);
        }

        use std::cmp::Ordering::*;

        if let [score1, score2, ..] = scores[..] {
            match score1.cmp(&score2) {
                Less => 2,
                Greater => 1,
                Equal => 0,
            }
        } else {
            0
        }
    }
}

/// 2760 Longest Even Odd Subarray With Threshold
struct Sol2760 {}

impl Sol2760 {
    pub fn longest_alternating_subarray(nums: Vec<i32>, threshold: i32) -> i32 {
        nums.chunk_by(|&a, &b| a & 1 != b & 1 && a <= threshold && b <= threshold)
            .fold(0, |longest, chunk| {
                println!("-> {chunk:?}");

                if chunk[0] & 1 == 0 {
                    if chunk[0] <= threshold {
                        chunk.len().max(longest)
                    } else {
                        longest
                    }
                } else if chunk.len() > 1 {
                    longest.max(chunk.len() - 1)
                } else {
                    longest
                }
            }) as _
    }
}

/// 2765 Longest Alternating Subarray
struct Sol2765 {}

impl Sol2765 {
    pub fn alternating_subarray(nums: Vec<i32>) -> i32 {
        let longest = nums.iter().enumerate().fold(0, |longest, (i, _)| {
            let mut cur = 0;
            for (w, &diff) in nums
                .iter()
                .skip(i)
                .collect::<Vec<_>>()
                .windows(2)
                .zip([1, -1].iter().cycle())
            {
                println!("-> {i} {diff:>2} {w:?}");

                if w[1] - w[0] == diff {
                    cur += 1;
                } else {
                    break;
                }
            }

            longest.max(cur)
        });

        if longest == 0 {
            return -1;
        }

        (longest + 1) as _
    }
}

/// 2848 Points That Intersect With Cars
struct Sol2848 {}

impl Sol2848 {
    pub fn number_of_points(nums: Vec<Vec<i32>>) -> i32 {
        use std::collections::{BTreeMap, HashSet};

        let mut sweep = BTreeMap::new();

        let mut covered = HashSet::new();
        for car in nums {
            if let [start, end, ..] = car[..] {
                sweep.entry(start).and_modify(|f| *f += 1).or_insert(1);
                sweep.entry(end + 1).and_modify(|f| *f -= 1).or_insert(-1);

                for point in start..=end {
                    covered.insert(point);
                }
            }
        }
        println!("-> {covered:?}");

        println!(
            ":? {sweep:?} {:?}",
            sweep
                .iter()
                .collect::<Vec<_>>()
                .windows(2)
                .fold((0, 0), |(psum, points), w| {
                    let psum = psum + w[0].1;
                    if psum > 0 {
                        (psum, points + w[1].0 - w[0].0)
                    } else {
                        (psum, points)
                    }
                })
        );

        covered.len() as _
    }
}

/// 2899 Last Visited Integers
struct Sol2899 {}

impl Sol2899 {
    pub fn last_visited_integers(nums: Vec<i32>) -> Vec<i32> {
        use std::collections::VecDeque;

        let mut seen = VecDeque::new();
        let mut k = 0;

        nums.iter().fold(vec![], |mut lvis, &n| {
            match n {
                -1 => {
                    k += 1;
                    if k <= seen.len() {
                        lvis.push(seen[k - 1]);
                    } else {
                        lvis.push(-1);
                    }
                }
                _ => {
                    k = 0;
                    seen.push_front(n);
                }
            }

            lvis
        })
    }
}

/// 2903 Find Indices With Index and Value Difference I
struct Sol2903 {}

impl Sol2903 {
    pub fn find_indices(nums: Vec<i32>, index_difference: i32, value_difference: i32) -> Vec<i32> {
        for (i, a) in nums.iter().enumerate() {
            for (j, b) in nums.iter().enumerate().skip(i + index_difference as usize) {
                if (a - b).abs() >= value_difference {
                    return vec![i as i32, j as i32];
                }
            }
        }

        vec![-1, -1]
    }
}

/// 2908 Minimum Sum of Mountain Triplets I
struct Sol2908 {}

impl Sol2908 {
    pub fn minimum_sum(nums: Vec<i32>) -> i32 {
        let mut msum = i32::MAX;
        for (i, a) in nums.iter().enumerate() {
            for (j, b) in nums.iter().enumerate().skip(i + 1) {
                for c in nums.iter().skip(j + 1) {
                    if a < b && c < b {
                        msum = msum.min(a + b + c);
                    }
                }
            }
        }

        let mut left_min = nums[0];
        let mut right_mins = nums.iter().rev().enumerate().skip(1).fold(
            vec![nums[nums.len() - 1]],
            |mut right_mins, (i, &n)| {
                right_mins.push(right_mins[i - 1].min(n));
                right_mins
            },
        );
        right_mins.reverse();
        println!("-> {right_mins:?}");
        println!(
            ":? {:?}",
            nums.iter()
                .enumerate()
                .skip(1)
                .take(nums.len() - 2)
                .fold(i32::MAX, |msum, (i, &n)| {
                    if n > left_min && n > right_mins[i + 1] {
                        let cur = n + left_min + right_mins[i + 1];
                        left_min = left_min.min(n);

                        msum.min(cur)
                    } else {
                        left_min = left_min.min(n);

                        msum
                    }
                })
        );

        if msum < i32::MAX { msum } else { -1 }
    }
}

/// 2917 Find the K-or of an Array
struct Sol2917 {}

impl Sol2917 {
    pub fn find_k_or(nums: Vec<i32>, k: i32) -> i32 {
        let mut counts = [0; 32];

        for n in nums {
            for (p, count) in counts.iter_mut().enumerate() {
                *count += (n & 1 << p) >> p;
            }
        }

        counts
            .iter()
            .enumerate()
            .filter(|&(_, &f)| f >= k)
            .fold(0, |k_or, (p, _)| k_or | 1 << p) as _
    }
}

/// 2946 Matrix Similarity After Cyclic Shifts
struct Sol2946 {}

impl Sol2946 {
    pub fn are_similar(mat: Vec<Vec<i32>>, k: i32) -> bool {
        let k = k as usize % mat[0].len();

        mat.iter().enumerate().all(|(r, row)| {
            row.iter()
                .enumerate()
                .all(|(c, &n)| n == mat[r][(c + k) % mat[0].len()])
        })
    }
}

/// 2970 Count the Number of Incremovable Subarrays I
struct Sol2970 {}

impl Sol2970 {
    pub fn incremovable_subarray_count(nums: Vec<i32>) -> i32 {
        (0..nums.len())
            .flat_map(|l| (l..nums.len()).map(move |r| (l, r)))
            .filter(|&(l, r)| {
                println!("{:?}", (l, r));
                nums[..l]
                    .iter()
                    .chain(&nums[r + 1..])
                    .collect::<Vec<_>>()
                    .is_sorted_by(|a, b| a < b)
            })
            .count() as _
    }
}

/// 2974 Minimum Number Game
struct Sol2974 {}

impl Sol2974 {
    pub fn number_game(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort();
        nums.chunks(2)
            .flat_map(|chunk| [chunk[1], chunk[0]])
            .collect()
    }
}

/// 3028 Ant on the Boundary
struct Sol3028 {}

impl Sol3028 {
    pub fn return_to_boundary_count(nums: Vec<i32>) -> i32 {
        nums.iter()
            .scan(0, |psum, &n| {
                *psum += n;
                Some(*psum)
            })
            .filter(|&n| n == 0)
            .count() as _
    }
}

/// 3200 Maximum Height of a Triangle
struct Sol3200 {}

impl Sol3200 {
    pub fn max_height_of_triangle(red: i32, blue: i32) -> i32 {
        [(red, blue), (blue, red)]
            .into_iter()
            .map(|(red, blue)| {
                let (mut r, mut b) = (0, 0);
                let mut height = 0;
                for (n, color) in (1..).zip(['r', 'b'].iter().cycle()) {
                    match color {
                        'r' => {
                            r += n;
                            if r <= red {
                                height += 1;
                            } else {
                                break;
                            }
                        }
                        'b' => {
                            b += n;
                            if b <= blue {
                                height += 1;
                            } else {
                                break;
                            }
                        }
                        _ => {}
                    }

                    println!("-> {n} {color:?} :: {height}");
                }
                height
            })
            .max()
            .unwrap_or(0)
    }
}

/// 3127 Make a Square with the Same Color
struct Sol3127 {}

impl Sol3127 {
    pub fn can_make_square(grid: Vec<Vec<char>>) -> bool {
        !(0..2).all(|r| {
            (0..2).all(|c| {
                grid.iter().skip(r).take(2).fold(0, |count, row| {
                    row.iter()
                        .skip(c)
                        .take(2)
                        .filter(|&&color| color == 'W')
                        .count()
                        + count
                }) == 2
            })
        })
    }
}

/// 3349 Adjacent Increasing Subarrays Detection I
struct Sol3349 {}

impl Sol3349 {
    pub fn has_increasing_subarrays(nums: Vec<i32>, k: i32) -> bool {
        let k = k as usize;

        println!(
            ":? {}",
            nums.windows(k)
                .zip(nums.windows(k).skip(k))
                .any(|(w1, w2)| w1.is_sorted_by(|a, b| a < b) && w2.is_sorted_by(|a, b| a < b))
        );

        (0..=nums.len() - 2 * k).any(|start| {
            nums.iter().skip(start).take(k).is_sorted_by(|a, b| a < b)
                && nums
                    .iter()
                    .skip(start + k)
                    .take(k)
                    .is_sorted_by(|a, b| a < b)
        })
    }
}

/// 3353 Minimum Total Operations
struct Sol3353 {}

impl Sol3353 {
    /// 1 <= N <= 10^5
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        nums.iter()
            .take(nums.len() - 1)
            .rev()
            .skip_while(|n| *n == nums.last().unwrap())
            .collect::<Vec<_>>()
            .chunk_by(|x, y| x == y)
            .count() as _
    }
}

/// 3386 Button with Longest Push Time
struct Sol3386 {}

impl Sol3386 {
    pub fn button_with_longest_time(events: Vec<Vec<i32>>) -> i32 {
        events.windows(2).fold(events[0].clone(), |longest, w| {
            println!("-> {longest:?} {w:?}");

            let diff = w[1][1] - w[0][1];
            if diff > longest[1] || diff == longest[1] && w[1][0] < longest[0] {
                vec![w[1][0], diff]
            } else {
                longest
            }
        })[0]
    }
}

/// 3417 Zigzag Grid Traversal With Skip
struct Sol3417 {}

impl Sol3417 {
    pub fn zigzag_traversal(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let mut tzz = grid
            .iter()
            .step_by(2)
            .zip(grid.iter().skip(1).step_by(2))
            .fold(vec![], |mut tzz, (e_row, o_row)| {
                for &n in e_row.iter().chain(o_row.iter().rev()).step_by(2) {
                    tzz.push(n);
                }

                tzz
            });

        if grid.len() & 1 == 1 {
            for &n in grid.last().unwrap().iter().step_by(2) {
                tzz.push(n);
            }
        }

        tzz
    }
}

/// 3432 Count Partitions with Even Sum Difference
struct Sol3432 {}

impl Sol3432 {
    pub fn count_partitions(nums: Vec<i32>) -> i32 {
        let tsum = nums.iter().sum::<i32>();
        println!(":? {}", if tsum & 1 == 0 { nums.len() - 1 } else { 0 });

        let mut sfx = nums.iter().sum::<i32>();
        let mut pfx = 0;

        nums.iter().take(nums.len() - 1).fold(0, |count, &n| {
            sfx -= n;
            pfx += n;

            if (sfx - pfx) & 1 == 0 {
                count + 1
            } else {
                count
            }
        }) as _
    }
}

/// 3452 Sum of Good Numbers
struct Sol3452 {}

impl Sol3452 {
    pub fn sum_of_good_numbers(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;

        nums.iter()
            .enumerate()
            .filter(|&(i, &n)| {
                nums.get(i + k).is_none_or(|&x| x < n)
                    && nums.get(i.wrapping_sub(k)).is_none_or(|&x| x < n)
            })
            .map(|(_, &n)| n)
            .sum()
    }
}

/// 3502 Minimum Cost to Reach Every Person
struct Sol3502 {}

impl Sol3502 {
    pub fn min_costs(cost: Vec<i32>) -> Vec<i32> {
        cost.iter()
            .scan(i32::MAX, |m, &c| {
                *m = c.min(*m);
                Some(*m)
            })
            .collect()
    }
}

/// 3507 Minimum Pair Removal to Sort Array I
struct Sol3507 {}

impl Sol3507 {
    pub fn minimum_pair_removal(mut nums: Vec<i32>) -> i32 {
        let mut ops = 0;
        while !nums.is_sorted() {
            if let Some((m_sum, m_index)) = nums
                .windows(2)
                .enumerate()
                .map(|(i, w)| (w.iter().sum::<i32>(), i))
                .min()
            {
                nums.splice(m_index..m_index + 2, [m_sum]);
            }

            println!("-> {nums:?}");
            ops += 1;
        }

        ops
    }
}

/// 3550 Smallest Index With Digit Sum Equal to Index
struct Sol3550 {}

impl Sol3550 {
    pub fn smallest_index(nums: Vec<i32>) -> i32 {
        println!(
            ":? {}",
            nums.iter()
                .enumerate()
                .find(|&(i, &n)| n
                    .to_string()
                    .chars()
                    .map(|chr| chr.to_digit(10).unwrap_or(0))
                    .sum::<u32>() as usize
                    == i)
                .map(|(i, _)| i as i32)
                .unwrap_or(-1)
        );

        nums.iter()
            .enumerate()
            .filter(|&(i, &n)| {
                let mut dsum = 0;
                let mut n = n;
                while n > 0 {
                    dsum += n % 10;
                    n /= 10;
                }
                dsum as usize == i
            })
            .nth(0)
            .map(|(i, _)| i as i32)
            .unwrap_or(-1)
    }
}

/// 3663 Find The Least Frequent Digit
struct Sol3663 {}

impl Sol3663 {
    pub fn get_least_frequent_digit(n: i32) -> i32 {
        let mut freqs = [0; 10];

        let mut n = n as usize;
        while n > 0 {
            freqs[n % 10] += 1;
            n /= 10;
        }

        freqs
            .into_iter()
            .enumerate()
            .filter(|&(_, f)| f > 0)
            .min_by(|s, t| s.1.cmp(&t.1))
            .unwrap()
            .0 as _
    }
}

/// 3683 Earliest Time to Finish One Task
struct Sol3683 {}

impl Sol3683 {
    pub fn earliest_time(tasks: Vec<Vec<i32>>) -> i32 {
        tasks.iter().map(|v| v.iter().sum()).min().unwrap()
    }
}

#[cfg(test)]
mod tests;
