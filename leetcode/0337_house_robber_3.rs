use std::cell::RefCell;
use std::rc::Rc;
type TreeNodeFull = Option<Rc<RefCell<TreeNode>>>;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: TreeNodeFull,
    pub right: TreeNodeFull,
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


pub fn rob_complex(root: TreeNodeFull) -> (i32, i32) {
    if root.is_none() {
        return (0, 0);
    }
    let root = root.unwrap();
    let root= root.borrow_mut();
    let result_l = rob_complex( root.left.clone());
    let result_r = rob_complex( root.right.clone());
    let score_with_me = root.val + result_l.1 + result_r.1;
    let score_without_me = result_l.0.max(result_l.1) + result_r.0.max(result_r.1);
    // println!("{score_with_me},{score_without_me} {:?}", root);
    (score_with_me, score_without_me)
}

pub fn rob(root: TreeNodeFull) -> i32 {
    let result = rob_complex(root);
    result.0.max(result.1)
}

pub fn wrap(node: TreeNode) -> TreeNodeFull {
    Some(Rc::new(RefCell::new(node)))
}

pub fn main() {
    // let tlr = TreeNode {
    //     val: 3,
    //     left: None,
    //     right: None,
    // };
    // let tl = TreeNode {
    //     val: 2,
    //     left: None,
    //     right: wrap(tlr),
    // };
    // let trr = TreeNode {
    //     val: 1,
    //     left: None,
    //     right: None,
    // };
    // let tr = TreeNode {
    //     val: 3,
    //     left: None,
    //     right: wrap(trr),
    // };
    // let l1 = TreeNode {
    //     val: 3,
    //     left: wrap(tl),
    //     right: wrap(tr),
    // };
    
    // let x = rob(wrap(l1));
    // println!("{}", x);

    let tlll = TreeNode {
        val: 3,
        left: None,
        right: None,
    };
    let tll = TreeNode {
        val: 2,
        left: wrap(tlll),
        right: None,
    };
    let tl = TreeNode {
        val: 1,
        left: wrap(tll),
        right: None,
    };
    let t = TreeNode {
        val: 4,
        left: wrap(tl),
        right: None,
    };
    
    let x = rob(wrap(t));
    println!("{}", x);
}
