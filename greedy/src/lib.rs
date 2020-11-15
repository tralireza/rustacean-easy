//! # Greedy

/// 1196 How Many Apples Can You Put into the Basket
struct Sol1196 {}

impl Sol1196 {
    /// 1 <= W_i <= 1000
    pub fn max_number_of_apples(mut weight: Vec<i32>) -> i32 {
        {
            use std::cmp::Reverse;
            use std::collections::BinaryHeap;

            // O(N + K*log(N))
            let weight: BinaryHeap<Reverse<i32>> = weight.iter().map(|&w| Reverse(w)).collect();

            println!("-> W Heap: {weight:?}");
            println!(
                ":? O(N + K*log(N)): {}",
                weight
                    .iter()
                    .scan(0, |total_w, Reverse(w)| {
                        *total_w += w;
                        if *total_w <= 5000 { Some(1) } else { None }
                    })
                    .sum::<i32>()
            );
        }

        {
            let freqs = weight.iter().fold([0; 1000 + 1], |mut freqs, &w| {
                freqs[w as usize] += 1;
                freqs
            });
            println!("-> W Frequency: {freqs:?}...");

            println!(
                ":? O(N): {}",
                freqs
                    .iter()
                    .enumerate()
                    .filter(|&(_, &f)| f > 0)
                    .scan(5000, |capacity, (w, &f)| {
                        if *capacity < w {
                            None
                        } else {
                            let f = f.min(*capacity / w);
                            *capacity -= f * w;
                            Some(f)
                        }
                    })
                    .sum::<usize>()
            );
        }

        weight.sort(); // O(N*log(N))
        weight
            .iter()
            .scan(0, |total_w, w| {
                *total_w += w;
                if *total_w <= 5000 { Some(1) } else { None }
            })
            .sum::<i32>()
    }
}

/// 2511 Maximum Enemy Forts That Can Be Captured
struct Sol2511 {}

impl Sol2511 {
    pub fn capture_forts(mut forts: Vec<i32>) -> i32 {
        let mut xforts = 0;

        for _ in 0..2 {
            xforts = xforts.max(
                forts
                    .iter()
                    .enumerate()
                    .filter(|(_, f)| **f == 1)
                    .filter_map(|(i, _)| {
                        forts[i + 1..]
                            .iter()
                            .enumerate()
                            .skip_while(|(_, f)| **f == 0)
                            .take(1)
                            .next()
                    })
                    .filter(|p| *p.1 == -1)
                    .map(|p| p.0)
                    .max()
                    .unwrap_or(0),
            );

            forts.reverse();
        }

        xforts as _
    }
}

#[cfg(test)]
mod tests;
