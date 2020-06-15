//! # Greedy

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
