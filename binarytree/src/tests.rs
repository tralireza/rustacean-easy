use super::*;

#[test]
fn test_2265() {
    fn build_tree(start: usize, vals: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if vals.len() <= start || vals[start] == -1 {
            return None;
        }

        let mut n = TreeNode::new(vals[start]);
        n.left = build_tree(2 * start + 1, vals);
        n.right = build_tree(2 * start + 2, vals);

        Some(Rc::new(RefCell::new(n)))
    }

    for (rst, root) in [
        (5, build_tree(0, &[4, 8, 5, 0, 1, -1, 6][..])),
        (1, build_tree(0, &[1][..])),
    ] {
        println!("* {root:?}");
        assert_eq!(Sol2265::average_of_subtree(root), rst);
        println!(":: {rst:?}");
    }
}
