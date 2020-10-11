use super::*;

#[test]
fn test_1474() {
    for (rst, head, m, n) in [
        (
            vec![1, 2, 6, 7, 11, 12],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13],
            2,
            3,
        ),
        (vec![1, 5, 9], vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11], 1, 3),
    ] {
        type ListNode = ListNode1474;

        fn ll_build(start: usize, vals: &[i32]) -> Option<Box<ListNode>> {
            if start == vals.len() {
                return None;
            }

            Some(Box::new(ListNode {
                val: vals[start],
                next: ll_build(start + 1, vals),
            }))
        }

        let head = ll_build(0, &head);
        let rst = ll_build(0, &rst);

        println!("* {head:?} {m} {n}");
        assert_eq!(Sol1474::delete_nodes(head, m, n), rst);
        println!(":: {rst:?}");
    }
}

#[test]
fn test_3062() {
    for (rst, head) in [
        (
            "Even",
            Some(Box::new(ListNode3062 {
                val: 2,
                next: Some(Box::new(ListNode3062::new(1))),
            })),
        ),
        (
            "Odd",
            Some(Box::new(ListNode3062 {
                val: 2,
                next: Some(Box::new(ListNode3062 {
                    val: 5,
                    next: Some(Box::new(ListNode3062 {
                        val: 4,
                        next: Some(Box::new(ListNode3062 {
                            val: 7,
                            next: Some(Box::new(ListNode3062 {
                                val: 20,
                                next: Some(Box::new(ListNode3062 { val: 5, next: None })),
                            })),
                        })),
                    })),
                })),
            })),
        ),
        (
            "Tie",
            Some(Box::new(ListNode3062 {
                val: 4,
                next: Some(Box::new(ListNode3062 {
                    val: 5,
                    next: Some(Box::new(ListNode3062 {
                        val: 2,
                        next: Some(Box::new(ListNode3062::new(1))),
                    })),
                })),
            })),
        ),
    ] {
        println!("* {head:?}");
        assert_eq!(Sol3062::game_result(head), rst);
        println!(":: {rst:?}");
    }
}
