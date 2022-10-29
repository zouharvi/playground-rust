type TreeNodeFull = Option<Rc<RefCell<TreeNode>>>;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;
pub fn is_valid_bst_local(
    root: &Option<Rc<RefCell<TreeNode>>>,
) -> (bool, Option<i32>, Option<i32>) {
    if root.is_none() {
        return (true, None, None);
    }
    let root = &root.clone().unwrap();
    let root = root.borrow();
    let (l_balanced, l_min, l_max) = is_valid_bst_local(&root.left);
    let (r_balanced, r_min, r_max) = is_valid_bst_local(&root.right);
    if !l_balanced || !r_balanced {
        return (false, None, None);
    }

    if let Some(r_min) = r_min {
        if r_min <= root.val {
            return (false, None, None);
        }
    }

    if let Some(l_max) = l_max {
        if l_max >= root.val {
            return (false, None, None);
        }
    }
    let l_min = l_min.unwrap_or(root.val);
    let r_max = r_max.unwrap_or(root.val);
    return (true, Some(l_min), Some(r_max));
}

pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    return is_valid_bst_local(&root).0;
}

pub fn wrap(node: TreeNode) -> TreeNodeFull {
    Some(Rc::new(RefCell::new(node)))
}

pub fn main() {
    let tlr = TreeNode {
        val: 3,
        left: None,
        right: None,
    };
    let tl = TreeNode {
        val: 2,
        left: None,
        right: wrap(tlr),
    };
    let trr = TreeNode {
        val: 1,
        left: None,
        right: None,
    };
    let tr = TreeNode {
        val: 3,
        left: None,
        right: wrap(trr),
    };
    let l1 = TreeNode {
        val: 3,
        left: wrap(tl),
        right: wrap(tr),
    };

    let x = is_valid_bst_local(&wrap(l1));

    println!("{:?}", x);
    let tl = TreeNode {
        val: 1,
        left: None,
        right: None,
    };
    let tr = TreeNode {
        val: 3,
        left: None,
        right: None,
    };
    let l1 = TreeNode {
        val: 2,
        left: wrap(tl),
        right: wrap(tr),
    };

    let x = is_valid_bst_local(&wrap(l1));
    println!("{:?}", x);
    let tl = TreeNode {
        val: 2,
        left: None,
        right: None,
    };
    let tr = TreeNode {
        val: 2,
        left: None,
        right: None,
    };
    let l1 = TreeNode {
        val: 2,
        left: wrap(tl),
        right: wrap(tr),
    };

    let x = is_valid_bst_local(&wrap(l1));
    println!("{:?}", x);
}
