#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

// report maximum depth, self-depth
pub fn middle_node_complex(
    head: Option<Box<ListNode>>,
    depth: i32,
) -> (i32, Option<Box<ListNode>>) {
    if head.is_none() {
        return (depth-1, None);
    }
    let head = head.unwrap();
    let (max_depth, result) = middle_node_complex(head.clone().next, depth + 1);
    if result.is_some() {
        return (max_depth, result);
    }
    if ((max_depth as f32) / 2.0).ceil() as i32 == depth {
        return (max_depth, Some(head));
    }
    (max_depth, None)
}

pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    middle_node_complex(head, 0).1
}

pub fn main() {
    let l2 = ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode {
                        val: 5,
                        next: Some(Box::new(ListNode { val: 3, next: None })),
                        // next: None,
                    })),
                })),
            })),
        })),
    };
    println!("{:?}", middle_node(Some(Box::new(l2))).unwrap().val);
    let l2 = ListNode {
        val: 1,
        next: None
    };
    println!("{:?}", middle_node(Some(Box::new(l2))).unwrap().val);
    let l2 = ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: None
        })),
    };
    println!("{:?}", middle_node(Some(Box::new(l2))).unwrap().val);
}
