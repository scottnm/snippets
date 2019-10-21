-- ?> compress "aaaabccaadeeee"
-- "abcade"

compress (x:(y:ys))
  | x == y = compress (y:ys)
  | otherwise = x : (compress (y:ys))
compress xs = xs

