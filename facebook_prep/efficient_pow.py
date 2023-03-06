def power(x, n):
  if n < 0:
      return 1/power(x, -n)
  if n == 1:
      return x
  if n == 0:
      return 1

  x_to_ndiv2 = power(x, n//2)
  if n % 2 == 1:
      return x_to_ndiv2*x_to_ndiv2*x
  else:
    return x_to_ndiv2*x_to_ndiv2