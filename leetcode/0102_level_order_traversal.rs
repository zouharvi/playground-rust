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

pub fn level_order_depth(root: Option<&Rc<RefCell<TreeNode>>>, out: &mut Vec<Vec<i32>>, depth: usize) {
    if root.is_none() {
        return;
    }

    if depth >= out.len() {
        out.push(Vec::<i32>::new());
    }

    let root = root.unwrap();
    let root = root.borrow();
    out[depth].push(root.val);

    level_order_depth(root.left.as_ref(), out, depth+1);
    level_order_depth(root.right.as_ref(), out, depth+1);
}

pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    if root.is_none() {
        return [].to_vec();
    }
    let mut out = Vec::<Vec<i32>>::new();
    level_order_depth(root.as_ref(), &mut out, 0);
    out
}

pub fn main() {
    // println!("{}", function_name(5));
}
