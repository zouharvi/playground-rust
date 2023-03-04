N = 5
QUERIES = [
  (2, 3),
  (1, 2),
  (2, 1),
  (2, 3),
  (2, 2),
]

def mutate_state(pos, state, nearest_map):
  # do nothing
  if state[pos-1]:
    return
  
  # variable is passed by reference and `state` is mutated in-place
  state[pos-1] = True

  # TODO: insert at the exact position instead of sorting, which will cost only O(N)
  nearest_map.append(pos-1)
  nearest_map.sort()

def find_lowest_geq(pos, nearest_map):
  # offset to be 0-indexed
  pos -= 1
  if not nearest_map:
    return -1
  
  best_candidate = nearest_map[-1]
  if best_candidate < pos:
    return -1
  
  
  # left and right are always valid indicies
  left = 0
  right = len(nearest_map)-1
  while True:
    mid = (right-left) // 2
    if nearest_map[mid] == pos:
      return pos+1
    # we're too high but still a better candidate
    elif nearest_map[mid] > pos:
      right = mid
      best_candidate = min(nearest_map[mid], best_candidate)
    # we're too low which is unacceptable
    elif nearest_map[mid] < pos:
      left = mid

    # make sure we terminate
    if left == right:
      # TODO: maybe not necessary
      if nearest_map[mid] > pos:
        best_candidate = min(nearest_map[mid], best_candidate)
      break
      
  return best_candidate+1

def retrieve_neighbour(pos, state, nearest_map):
  for state_i, state_val in enumerate(state[pos-1:]):
    if state_val:
      return pos-1+state_i+1
  return -1

def processor(QUERIES):
  state = [False]*N
  nearest_map = []
  for (query_type, query_pos) in QUERIES:
    print(state)
    print(nearest_map)
    if query_type == 1:
      mutate_state(query_pos, state, nearest_map)
    elif query_type == 2:
      print(find_lowest_geq(query_pos, nearest_map))
    else:
      raise Exception(f"Unkown command {query_type}")
    
processor(QUERIES)