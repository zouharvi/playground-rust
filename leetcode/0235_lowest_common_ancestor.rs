use std::cell::RefCell;
use std::rc::Rc;

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

// This is asymptotically slower than the optimal
pub fn find_node(root: Rc<RefCell<TreeNode>>, node: Rc<RefCell<TreeNode>>) -> Vec<Rc<RefCell<TreeNode>>> {
    let root_val = root.borrow().val;
    let node_val = node.borrow().val;
    let mut out: Vec<_> = [root.clone()].to_vec();
    loop {
        if node_val == root_val {
            return out;
        } else if node_val < root_val {
            let mut result = find_node(
                root.borrow().left.as_ref().map(|c| c.clone()).unwrap(),
                node,
            );
            out.append(&mut result);
            return out;
        } else if node_val > root_val {
            let mut result = find_node(
                root.borrow().right.as_ref().map(|c| c.clone()).unwrap(),
                node,
            );
            out.append(&mut result);
            return out;
        }
    }
}

pub fn lowest_common_ancestor(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    let root = root.unwrap();
    let parents_p = find_node(root.clone(), p.unwrap());
    let parents_q = find_node(root, q.unwrap());
    let mut last_good = parents_p[0].clone();
    for (p, q) in parents_p.iter().zip(parents_q.iter()) {
        if p.borrow().val != q.borrow().val {
            return Some(last_good);
        }
        last_good = p.clone();
    }
    None
}

pub fn wrap(node: TreeNode) -> Option<Rc<RefCell<TreeNode>>> {
    Some(Rc::new(RefCell::new(node)))
}
pub fn main() {
    let tlr = TreeNode {
        val: 4,
        left: None,
        right: None,
    };
    let tll = TreeNode {
        val: 0,
        left: None,
        right: None,
    };
    let tl = TreeNode {
        val: 2,
        left: wrap(tll),
        right: wrap(tlr),
    };
    let trl = TreeNode {
        val: 7,
        left: None,
        right: None,
    };
    let trr = TreeNode {
        val: 9,
        left: None,
        right: None,
    };
    let tr = TreeNode {
        val: 8,
        left: wrap(trl),
        right: wrap(trr),
    };
    let l1 = TreeNode {
        val: 6,
        left: wrap(tl),
        right: wrap(tr),
    };

    let a = TreeNode {
        val: 0,
        left: None,
        right: None,
    };
    let b = TreeNode {
        val: 4,
        left: None,
        right: None,
    };

    println!("{:?}", lowest_common_ancestor(wrap(l1), wrap(a), wrap(b)).unwrap().borrow().val);
}
