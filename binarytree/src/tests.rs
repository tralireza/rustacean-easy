use super::*;

#[test]
fn test_2265() {
    for (rst, root) in [(
        1,
        Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        }))),
    )] {
        println!("* {root:?}");
        assert_eq!(Sol2265::average_of_subtree(root), rst);
        println!(":: {rst:?}");
    }
}
