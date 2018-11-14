module Prime (nth) where

prime x = not (elem 0 (map (x `mod`) [2..x-1]))

nth :: Int -> Maybe Integer
nth 0 = Nothing
nth n = Just (last (take n (filter (prime) [2..])))
