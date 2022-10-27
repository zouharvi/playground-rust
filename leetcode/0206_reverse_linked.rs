#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
}

pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() {
        return head
    }

    let mut r_head = head.unwrap();
    head = r_head.next;
    r_head.next = None;
    
    while head.is_some() {
        let mut head_c = head.take().unwrap();
        // this is gonna be the next head
        let tmp = head_c.next;
        head_c.next = Some(r_head);
        r_head = head_c;
        head = tmp;
    }

    Some(r_head)
}

pub fn main() {
    let l1 = ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode { val: 4, next: None })),
        })),
    };
    let x = reverse_list(Some(Box::new(l1)));
    println!("{:?}", x);
}