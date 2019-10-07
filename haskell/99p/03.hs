myLastA :: [a] -> a
myLastA [] = error "empty list provided!"
myLastA list = list !! ((length list) - 1)

ftranElemAt :: [a] -> Int -> a
ftranElemAt list index
  | index > (length list) = error "index out of bounds"
  | index == 1            = head list
  | index <= 0            = error "index must be at least 1"
  | otherwise             = ftranElemAt (tail list) (index -1)

