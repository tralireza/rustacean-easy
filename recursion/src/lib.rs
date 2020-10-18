//! # Recursion

/// 37h Sudoku Solver
struct Sol37 {}

impl Sol37 {
    pub fn solve_sudoku(board: &mut [Vec<char>]) {
        fn valid(board: &[Vec<char>]) -> bool {
            let (mut rows, mut cols, mut cells) = ([0; 9], [0; 9], [0; 9]);

            for (r, row) in board.iter().enumerate() {
                for (c, mask) in row
                    .iter()
                    .enumerate()
                    .filter(|&(_, &chr)| chr != '.')
                    .map(|(c, chr)| (c, 1 << (chr.to_digit(10).unwrap() - 1)))
                {
                    if rows[r] & mask == mask {
                        return false;
                    }
                    rows[r] |= mask;

                    if cols[c] & mask == mask {
                        return false;
                    }
                    cols[c] |= mask;

                    if cells[3 * (r / 3) + c / 3] & mask == mask {
                        return false;
                    }
                    cells[3 * (r / 3) + c / 3] |= mask;
                }
            }

            true
        }

        fn solve(r: usize, c: usize, board: &mut [Vec<char>]) -> bool {
            if r == 9 {
                return true;
            }

            if board[r][c] != '.' {
                return solve(r + (c + 1) / 9, (c + 1) % 9, board);
            }

            for chr in '1'..='9' {
                board[r][c] = chr;
                if valid(board) && solve(r + (c + 1) / 9, (c + 1) % 9, board) {
                    return true;
                }
                board[r][c] = '.';
            }

            false
        }

        solve(0, 0, board);
    }
}

/// 1066m Compus Bikes II
struct Sol1066 {}

impl Sol1066 {
    /// 1 <= Workers <= Bikes <= 10
    pub fn assign_bikes(workers: Vec<Vec<i32>>, bikes: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashMap;

        let mut cache = vec![vec![None; 1 << bikes.len()]; workers.len()];
        let mut hm_cache: HashMap<(usize, usize), i32> = HashMap::new();

        fn search(
            w: usize,
            mut bike_mask: usize,
            workers: &[Vec<i32>],
            bikes: &[Vec<i32>],
            cache: &mut Vec<Vec<Option<i32>>>,
            hm_cache: &mut HashMap<(usize, usize), i32>,
        ) -> i32 {
            if w == workers.len() {
                return 0;
            }

            if let Some(dist) = cache[w][bike_mask] {
                return dist;
            }
            println!("-> {w} {bike_mask:>010b} ({bike_mask})");

            let worker = &workers[w];
            let mut dist = i32::MAX;
            for (b, bike) in bikes.iter().enumerate() {
                if bike_mask & (1 << b) == 0 {
                    bike_mask |= 1 << b;

                    let cur_dist = (worker[0] - bike[0]).abs()
                        + (worker[1] - bike[1]).abs()
                        + search(w + 1, bike_mask, workers, bikes, cache, hm_cache);
                    dist = dist.min(cur_dist);

                    bike_mask &= !(1 << b);
                }
            }

            cache[w][bike_mask] = Some(dist);
            hm_cache.insert((w, bike_mask), dist);

            dist
        }

        let dist = search(0, 0, &workers, &bikes, &mut cache, &mut hm_cache);
        println!("-> {cache:?}");
        println!("-> {hm_cache:?}");

        dist
    }
}

/// 3483
struct Sol3483 {}

impl Sol3483 {
    pub fn total_numbers(mut digits: Vec<i32>) -> i32 {
        use std::collections::HashSet;

        fn dsearch(r: i32, p: i32, digits: &mut [i32], pset: &mut HashSet<i32>) {
            if r == 3 {
                if p & 1 == 0 {
                    pset.insert(p);
                }
                return;
            }

            for i in 0..digits.len() {
                if digits[i] > 0 || digits[i] == 0 && p > 0 {
                    let d = digits[i];
                    digits[i] = -1;
                    dsearch(r + 1, 10 * p + d, digits, pset);
                    digits[i] = d;
                }
            }
        }

        fn search(r: i32, p: i32, digits: &[i32], usage: &mut [bool], pset: &mut HashSet<i32>) {
            if r == 3 {
                if p & 1 == 0 {
                    pset.insert(p);
                }
                return;
            }

            for (i, &d) in digits.iter().enumerate() {
                if !usage[i] && (d > 0 || d == 0 && p > 0) {
                    usage[i] = true;
                    search(r + 1, 10 * p + d, digits, usage, pset);
                    usage[i] = false;
                }
            }
        }

        let mut pset = HashSet::new();
        let mut usage = vec![false; digits.len()];

        dsearch(0, 0, &mut digits, &mut pset);
        println!(":? {pset:?}   {}", pset.len());

        pset.clear();
        search(0, 0, &digits, &mut usage, &mut pset);

        println!("-> {pset:?}");
        pset.len() as _
    }
}

#[cfg(test)]
mod tests;
