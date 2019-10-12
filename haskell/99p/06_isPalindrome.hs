-- isPalindrome1 :: [a] -> Bool
isPalindrome1 [] = True
isPalindrome1 [_] = True
isPalindrome1 x = ((head x) == (last x)) && ((isPalindrome1 . init . tail) x)

firstHalf :: [a] -> [a]
firstHalf x = take (length x) x

-- isPalindrome2 :: [a] -> Bool
isPalindrome2 [] = True
isPalindrome2 [a] = True
isPalindrome2 x = (firstHalf x) == ((firstHalf . reverse) x)
