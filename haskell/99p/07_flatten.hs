-- ?> flatten (Elem 5)
-- [5]
-- ?> flatten (List [Elem 1, List [Elem 2, List [Elem 3, Elem 4], Elem 5]])
-- [1,2,3,4,5]
-- ?> flatten (List [])
-- []

data NestedList a = Elem a | List [NestedList a]

myFlatten :: NestedList a -> [a]
myFlatten (List (x:xs)) = (myFlatten x) ++ (myFlatten (List xs))
myFlatten (List _) = []
myFlatten (Elem a) = [a]
