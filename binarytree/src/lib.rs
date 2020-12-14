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

impl TreeNode {
    fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

/// 1302m Deepest Leaves Sum
struct Sol1302 {}

impl Sol1302 {
    pub fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        use std::collections::BTreeMap;

        let mut dsum = BTreeMap::new();
        fn walk(dsum: &mut BTreeMap<usize, i32>, n: Option<Rc<RefCell<TreeNode>>>, d: usize) {
            if let Some(n) = n {
                let n = n.borrow();
                dsum.entry(d)
                    .and_modify(|sum| *sum += n.val)
                    .or_insert(n.val);

                walk(dsum, n.left.clone(), d + 1);
                walk(dsum, n.right.clone(), d + 1);
            }
        }

        walk(&mut dsum, root, 0);

        println!("-> {dsum:?}");

        *dsum.last_entry().unwrap().get()
    }
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
                None => (0, 0, 0),
            }
        }

        fn walk_cloned(n: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32, i32) {
            n.map_or((0, 0, 0), |n| {
                let n = n.borrow();
                let (lcount, lsum, lx) = walk_cloned(n.left.clone());
                let (rcount, rsum, rx) = walk_cloned(n.right.clone());

                let mut x = lx + rx;
                if n.val == (lsum + rsum + n.val) / (lcount + rcount + 1) {
                    x += 1;
                }

                (lcount + rcount + 1, lsum + rsum + n.val, x)
            })
        }
        println!(":? {}", walk_cloned(root.clone()).2);

        walk(&root).2
    }
}

#[cfg(test)]
mod tests;
