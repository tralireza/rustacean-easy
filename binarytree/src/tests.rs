use super::*;

#[test]
fn test_1302() {
    fn build_tree(start: usize, vals: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        vals.get(start).map_or(None, |&val| {
            if val == -1 {
                None
            } else {
                Some(Rc::new(RefCell::new(TreeNode {
                    val,
                    left: build_tree(2 * start + 1, vals),
                    right: build_tree(2 * start + 2, vals),
                })))
            }
        })
    }

    for (rst, root) in [
        (
            15,
            &[1, 2, 3, 4, 5, -1, 6, 7, -1, -1, -1, -1, -1, -1, 8][..],
        ),
        (19, &[6, 7, 8, 2, 7, 1, 3, 9, -1, 1, 4, -1, -1, -1, 5][..]),
    ] {
        let root = build_tree(0, root);

        println!("* {root:?}");
        assert_eq!(Sol1302::deepest_leaves_sum(root), rst);
        println!(":: {rst:?}");
    }
}

#[test]
fn test_2265() {
    fn build_tree(start: usize, vals: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        println!("-> {start:>2}");

        vals.get(start).map_or(None, |&val| {
            if val == -1 {
                None
            } else {
                Some(Rc::new(RefCell::new(TreeNode {
                    val,
                    left: build_tree(2 * start + 1, vals),
                    right: build_tree(2 * start + 2, vals),
                })))
            }
        })
    }

    for (rst, root) in [
        (5, build_tree(0, &[4, 8, 5, 0, 1, -1, 6][..])),
        (1, Some(Rc::new(RefCell::new(TreeNode::new(1))))),
    ] {
        println!("* {root:?}");
        assert_eq!(Sol2265::average_of_subtree(root), rst);
        println!(":: {rst:?}");
    }
}
