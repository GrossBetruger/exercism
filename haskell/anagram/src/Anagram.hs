module Anagram (anagramsFor) where

import Data.Char
import Data.Map
import Data.List

lower s = Prelude.map (toLower) s
count c s = (c, length (Prelude.filter (==c) s))
count_all s = nub (Prelude.map (\x -> count x s) s)
contained w1 w2 = Data.Map.fromList(count_all (lower w1)) == Data.Map.fromList(count_all (lower w2))

anagramsFor :: String -> [String] -> [String]
anagramsFor xs xss = Prelude.filter (\word -> (lower word) /= (lower xs) && (contained xs word)) xss
