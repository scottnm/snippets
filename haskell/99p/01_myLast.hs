myLastA :: [a] -> a
myLastA [] = error "empty list provided!"
myLastA list = list !! ((length list) - 1)

myLastB :: [a] -> a
myLastB [x] = x
myLastB (_:xs) = myLastB xs
