use super::*;

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
