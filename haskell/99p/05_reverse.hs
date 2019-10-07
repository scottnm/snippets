-- my original implementation
myReverse :: [a] -> [a]
myReverse [] = []
myReverse list = (last list) : (myReverse (init list))

-- stolen from the 99p solutions to understand why my solution wasn't listed and to figure out what perf problems might exist
-- SPOILER: the issue (I believe) is the use of `last`
reverse2 :: [a] -> [a]
reverse2 list = reverse2' list []
  where
    reverse2' [] reversed     = reversed
    reverse2' (x:xs) reversed = reverse2' xs (x:reversed)
