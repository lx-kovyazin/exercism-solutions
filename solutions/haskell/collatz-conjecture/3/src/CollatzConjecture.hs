module CollatzConjecture (collatz) where

collatz :: Integer -> Maybe Integer
collatz x | x <= 0 = Nothing
          | x == 1 = Just 0
          | otherwise = let n | even x    = div x 2
                              | otherwise = 3 * x + 1
                        in Just (+1) <*> collatz n