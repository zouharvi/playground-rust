def binary_search_rotated(arr, key, left=0, right=None):
    if right == None:
        right = len(arr)-1

    mid = None
    while True:
        mid = (right+left)//2
        if arr[mid] == key:
            return mid

        if left == right:
            return -1

        if arr[mid] >= key:
            if arr[left] <= arr[right]:
                right = mid
            elif arr[right] < key:
                right = mid
            else:
                left = mid
                # left = mid
        if arr[mid] <= key:
            if arr[left] <= arr[right]:
                left = mid
            elif arr[left] > key:
                left = mid
            else:
                right = mid
                # right = mid



print(binary_search_rotated([0, 1, 2, 3, 4], 3))
print(binary_search_rotated([2, 3, 4, 0, 1], 3))
print(binary_search_rotated([2, 3, 4, 5, 0, 1], 3))
print(binary_search_rotated([2, 3, 4, 5, 0, 1], 0))


[0, 1, 2, 3, 4]
[2, 3, 4, 0, 1]