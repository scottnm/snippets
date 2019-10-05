doubleMe x = x * 2
addVals x y = x + y
rightTriangles maxSide perimeter = [(a, b, c) |
    c <- [1..maxSide], b <- [1..c], a <- [1..b],
    a^2+b^2 == c^2, 
    a + b + c <= perimeter ]
