myButLastA :: [a] -> a
myButLastA [] = error "not enough elements provided!"
myButLastA [_] = error "not enough elements provided!"
myButLastA list = list !! ((length list) - 2)

myButLastB :: [a] -> a
myButLastB = last . init
