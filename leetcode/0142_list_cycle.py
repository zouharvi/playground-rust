from typing import Optional

# Definition for singly-linked list.
class ListNode:
    def __init__(self, x):
        self.val = x
        self.next = None

def detectCycle(head: Optional[ListNode]) -> Optional[ListNode]:
    visited = dict()
    i = 0
    while True:
        if head is None:
            break
        h_hash = hash(head)
        if h_hash in visited:
            return visited[h_hash]
        visited[h_hash] = head
        i += 1
        head = head.next
    return None

if __name__ == "__main__":
    a = ListNode(3)
    b = ListNode(2)
    c = ListNode(0)
    d = ListNode(-4)
    a.next=b
    b.next=c
    c.next=d
    d.next=b
    print(detectCycle(a))
