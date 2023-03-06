def get_all_subsets(v):
  if len(v) == 0:
    return [[]]

  following_subsets = get_all_subsets(v[1:])
  sets = []
  for set in following_subsets:
    sets.append([v[0]] + set)
    sets.append(set)

  return sets

out = get_all_subsets([1, 2, 3, 4])
print(out)