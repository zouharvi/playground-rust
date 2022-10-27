from typing import List

class Node:
    def __init__(self, val=None, children=None):
        self.val = val
        self.children = children

def preorder(root: 'Node') -> List[int]:
    out = []
    if root is None:
        return out
    out.append(root.val)
    out += [v for c in root.children for v in preorder(c) ]
    return out