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
    type ListNode = ListNode3062;

    fn ll_build(start: usize, vals: &[i32]) -> Option<Box<ListNode>> {
        if start == vals.len() {
            None
        } else {
            Some(Box::new(ListNode {
                val: vals[start],
                next: ll_build(start + 1, vals),
            }))
        }
    }

    for (rst, head) in [
        ("Even", vec![2, 1]),
        ("Odd", vec![2, 5, 4, 7, 20, 5]),
        ("Tie", vec![4, 5, 2, 1]),
    ] {
        let head = ll_build(0, &head);

        println!("* {head:?}");
        assert_eq!(Sol3062::game_result(head), rst);
        println!(":: {rst:?}");
    }
}
