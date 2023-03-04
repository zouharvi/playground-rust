QUERIES = [
    (1, 42),
    (2, None),
    (1, 14),
    (3, None),
    (1, 28),
    (3, None),
    (1, 60),
    (1, 78),
    (2, None),
    (2, None),
]

def perform_enqueue(stack, val):
    # mutate in-place
    stack.append(val)

def perform_dequeue(stack):
    stack_bwd = []

    # transform everything to the second stack
    while stack:
        stack_bwd.append(stack.pop())

    # pop from the top of the new stack
    # TODO: assert is nonempty
    stack_bwd.pop()

    while stack_bwd:
        stack.append(stack_bwd.pop())

def perform_printfront(stack):
    stack_bwd = []

    # transform everything to the second stack
    while stack:
        stack_bwd.append(stack.pop())

    # print from the top of the new stack
    # TODO: assert is nonempty
    print(stack_bwd[-1])

    while stack_bwd:
        stack.append(stack_bwd.pop())

def processor(queries):
    # actually keep just one true stack and use the other one only temporarily
    stack_fwd = []

    for query_type, query_val in queries:
        if query_type == 1:
            perform_enqueue(stack_fwd, query_val)
        elif query_type == 2:
            perform_dequeue(stack_fwd)
        elif query_type == 3:
            perform_printfront(stack_fwd)
        
        

processor(QUERIES)