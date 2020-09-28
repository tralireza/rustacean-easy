use super::*;

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
