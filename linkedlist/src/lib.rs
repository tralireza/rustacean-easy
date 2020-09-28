//! # Linked List

/// 3062 Winner of the Linked List Game
#[derive(Debug)]
struct ListNode3062 {
    val: i32,
    next: Option<Box<ListNode3062>>,
}

impl ListNode3062 {
    fn new(val: i32) -> Self {
        ListNode3062 { val, next: None }
    }
}

struct Sol3062 {}

impl Sol3062 {
    pub fn game_result(head: Option<Box<ListNode3062>>) -> String {
        let (mut evens, mut odds) = (0, 0);

        let mut n = *head.unwrap();
        loop {
            let n_next = *n.next.unwrap();
            if n.val > n_next.val {
                evens += 1;
            } else if n.val < n_next.val {
                odds += 1;
            }

            if n_next.next.is_none() {
                break;
            }
            n = *n_next.next.unwrap();
        }

        use std::cmp::Ordering::*;
        match evens.cmp(&odds) {
            Greater => "Even",
            Less => "Odd",
            _ => "Tie",
        }
        .to_string()
    }
}

#[cfg(test)]
mod tests;
