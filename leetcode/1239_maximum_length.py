from typing import List


def maxLength(arr: List[str]) -> int:
    arr_s = [set(w) for w in arr]
    arr_s = [a_s for a_s, a in zip(arr_s, arr) if len(a_s) == len(a)]
    viable = [set()]
    largest = 0

    for a_s in arr_s:
        for v in viable:
            if len(v & a_s) != 0:
                continue
            v_new = v | a_s
            viable.append(v_new)
            largest = max(largest, len(v_new))

    return largest


print(maxLength(["un", "iq", "ue"]))
print(maxLength(["cha", "r", "act", "ers"]))
print(maxLength(["abcdefghijklmnopqrstuvwxyz"]))
print(maxLength(["aa", "bb"]))
print(maxLength(["a", "abc", "d", "de", "def"]))
print(maxLength([
    "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p"
]))
