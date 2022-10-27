


#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
}

pub fn merge_two_lists(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut l3_head = Box::new(ListNode {
        next: None,
        // nan value
        val: -1,
    });
    let mut l1 = l1;
    let mut l2 = l2;
    let mut l3 = &mut l3_head;

    while l1.is_some() && l2.is_some() {
        let l1_local = l1.take();
        let l2_local = l2.take();
        if let (Some(mut l1_head), Some(mut l2_head)) = (l1_local, l2_local) {
            if l1_head.val < l2_head.val {
                l1 = l1_head.next.take();
                l2 = Some(l2_head);
                l3 = l3.next.get_or_insert(l1_head);
            } else {
                l2 = l2_head.next.take();
                l1 = Some(l1_head);
                l3 = l3.next.get_or_insert(l2_head);
            }
        }
    }
    // set the leftover
    if l1.is_some() {
        l3.next = l1;
    } else {
        // either L2 is something or none, doesn't matter
        l3.next = l2;
    }

    l3_head.next
}

pub fn main() {
    let l1 = ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode { val: 4, next: None })),
        })),
    };
    let l2 = ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 3,
            next: Some(Box::new(ListNode { val: 4, next: None })),
        })),
    };
    let x = merge_two_lists(Some(Box::new(l1)), Some(Box::new(l2)));
    println!("{:?}", x);
}