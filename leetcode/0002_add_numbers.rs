// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
}

pub fn add_two_numbers(
    mut l1: Option<Box<ListNode>>,
    mut l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    if l1.is_some() && l2.is_none() {
        l2 = Some(Box::new(ListNode{val: 0, next: None}));
    }
    if l1.is_none() && l2.is_some() {
        l1 = Some(Box::new(ListNode{val: 0, next: None}));
    }
    if l1.is_none() && l2.is_none() {
        return None;
    }
    // l1 and l2 are Some from here on

    let mut l1 = l1.unwrap();
    let l2 = l2.unwrap();
    let v1 = l1.val;
    let v2 = l2.val;
    let v3 = (v1 + v2) % 10;
    let v_rest = (v1 + v2) / 10;

    let mut l3 = ListNode {
        val: v3,
        next: None,
    };

    if l1.next == None && v_rest != 0 {
        l1.next = Some(Box::new(ListNode { val: 0, next: None}));
    }
    if v_rest != 0 {
        let mut l1next = l1.next.unwrap(); 
        // we add the carry on to l1.next
        l1next.val += v_rest;
        let rest = add_two_numbers(Some(l1next), l2.next);
        l3.next = rest;
    } else {
        let rest = add_two_numbers(l1.next, l2.next);
        l3.next = rest;
    }
    Some(Box::new(l3))
}

pub fn main() {
    let l1 = ListNode {
        val: 2,
        next: Some(Box::new(ListNode {
            val: 4,
            next: Some(Box::new(ListNode {
                val: 3,
                next: None,
            })),
        })),
    };
    let l2 = ListNode {
        val: 5,
        next: Some(Box::new(ListNode {
            val: 6,
            next: Some(Box::new(ListNode {
                val: 4,
                next: None,
            })),
        })),
    };
    println!("{:?}", add_two_numbers(Some(Box::new(l1)), Some(Box::new(l2))));
}
