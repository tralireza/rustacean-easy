//! # Linked List

/// 1474 Delete N Nodes After M Nodes of a Linked List
#[derive(Debug, Clone, Eq, PartialEq)]
struct ListNode1474 {
    val: i32,
    next: Option<Box<ListNode1474>>,
}

impl ListNode1474 {}

struct Sol1474 {}

impl Sol1474 {
    pub fn delete_nodes(
        head: Option<Box<ListNode1474>>,
        m: i32,
        n: i32,
    ) -> Option<Box<ListNode1474>> {
        fn rdelete(
            head: &Option<Box<ListNode1474>>,
            m: i32,
            n: i32,
            counter: i32,
        ) -> Option<Box<ListNode1474>> {
            match head {
                None => None,
                Some(l) => {
                    if counter < m {
                        Some(Box::new(ListNode1474 {
                            val: l.val,
                            next: rdelete(&l.next, m, n, counter + 1),
                        }))
                    } else {
                        let mut next = l.next.clone();
                        for _ in 0..n {
                            match next {
                                None => next = None,
                                Some(x) => next = x.next,
                            }
                        }

                        Some(Box::new(ListNode1474 {
                            val: l.val,
                            next: rdelete(&next, m, n, 1),
                        }))
                    }
                }
            }
        }

        rdelete(&head, m, n, 1)
    }
}

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
        use std::cmp::Ordering::*;

        let (mut evens, mut odds) = (0, 0);

        let mut n = *head.unwrap();
        loop {
            let n_next = *n.next.unwrap();
            match n.val.cmp(&n_next.val) {
                Greater => evens += 1,
                Less => odds += 1,
                _ => (),
            }

            if n_next.next.is_none() {
                break;
            }
            n = *n_next.next.unwrap();
        }

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
