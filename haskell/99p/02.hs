myButLast :: [a] -> a
myButLast [] = error "not enough elements provided!"
myButLast [_] = error "not enough elements provided!"
myButLast list = list !! ((length list) - 2)
