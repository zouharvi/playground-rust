arr = [1, 1, 1, 2, 2, 2, 2, 2, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 6, 6, 6, 6, 6, 6]

def find_low_index(arr, key, left=0, right=None):
  # always point to valid
  if right == None:
    right = len(arr)-1

  while True:
    mid = (right+left)//2
    if arr[mid] == key and mid==0:
      return mid
    if arr[mid] == key and arr[mid-1] != key:
      return mid

    if left == right:
      return -1
    if arr[mid] >= key:
      right = mid
    if arr[mid] < key:
      left = mid

def find_high_index(arr, key, left=0, right=None):
  # always point to valid
  if right == None:
    right = len(arr)-1

  while True:
    mid = (right+left)//2
    if arr[mid] == key and mid==len(arr)-2:
      return mid
    if arr[mid] == key and arr[mid+1] != key:
      return mid

    if left == right:
      return -1
    if arr[mid] > key:
      right = mid
    if arr[mid] <= key:
      left = mid

# print(find_low_index(arr, 1))
print(find_low_index(arr, 2))
print(find_high_index(arr, 2))
print(find_low_index(arr, 6))
print(find_high_index(arr, 6))

# print(find_high_index(arr, 1))