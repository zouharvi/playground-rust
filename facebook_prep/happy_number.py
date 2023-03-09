def is_happy_number(num):
    prev_num = None
    while True:
        if num == prev_num:
            return False
        if num == 1:
            return True
        prev_num = num
        num = sum(int(x) for x in str(num))
        
print(5, is_happy_number(5))
print(1, is_happy_number(1))
print(115, is_happy_number(115))
print(100, is_happy_number(100))
