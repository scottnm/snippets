-- ?> pack ['a', 'a', 'a', 'a', 'b', 'c', 'c', 'a', 'a', 'd', 'e', 'e', 'e', 'e']
-- ["aaaa","b","cc","aa","d","eeee"]

import Debug.Trace

pack :: (Eq a) => [a] -> [[a]]
pack (x:xs)
  | x == rh   = trace "test1" ((x:r):rs)
  | otherwise = trace "test2" ([x]:(r:rs))
  where
    (r@(rh:_):rs) = pack(xs)
pack _ = []
