from typing import List

class Node:
    def __init__(self, val=None, children=None):
        self.val = val
        self.children = children

def postorder(root: 'Node') -> List[int]:
    out = []
    if root is None:
        return out
    out += [v for c in root.children for v in postorder(c) ]
    out.append(root.val)
    return out