module Anagram (anagramsFor) where

import Data.Map
import Data.List


count c s = (c, length (Prelude.filter (==c) s))
count_all s = nub (Prelude.map (\x -> count x s) s)
contained w1 w2 = Data.Map.fromList(count_all w1) == Data.Map.fromList(count_all w2)

anagramsFor :: String -> [String] -> [String]
anagramsFor xs xss = Prelude.filter (\word -> word /= xs && (contained xs word)) xss
