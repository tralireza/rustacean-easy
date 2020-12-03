//! # Binary Tree
//! # Binary Tree

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

/// 2265m Count Nodes Equal to Average of Subtree
struct Sol2265 {}

impl Sol2265 {
    pub fn average_of_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn walk(n: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32, i32) {
            match n {
                Some(n) => {
                    let n = n.borrow();
                    let (lcount, lsum, lx) = walk(&n.left);
                    let (rcount, rsum, rx) = walk(&n.right);

                    let mut x = lx + rx;
                    if n.val == (lsum + rsum + n.val) / (lcount + rcount + 1) {
                        x += 1;
                    }

                    (lcount + rcount + 1, lsum + rsum + n.val, x)
                }
                None => return (0, 0, 0),
            }
        }

        fn walk_cloned(n: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32, i32) {
            match n {
                Some(n) => {
                    let n = n.borrow();
                    let (lcount, lsum, lx) = walk_cloned(n.left.clone());
                    let (rcount, rsum, rx) = walk_cloned(n.right.clone());

                    let mut x = lx + rx;
                    if n.val == (lsum + rsum + n.val) / (lcount + rcount + 1) {
                        x += 1;
                    }

                    (lcount + rcount + 1, lsum + rsum + n.val, x)
                }
                _ => (0, 0, 0),
            }
        }
        println!(":? {}", walk_cloned(root.clone()).2);

        walk(&root).2
    }
}

#[cfg(test)]
mod tests;
