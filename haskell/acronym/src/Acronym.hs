module Acronym (abbreviate) where

import Data.Char


camel (c:cs) | length cs == 0 = [c]
             | (isUpper c && not (isUpper (head cs))) && isAlpha (head cs) = "  " ++ [c] ++ camel cs
             | otherwise = [c] ++ camel cs

replace_punct s = map (\c -> if elem c ",!.-" then ' ' else c) s

abbreviate :: String -> String
abbreviate xs = filter isAlpha $ map (toUpper) $ map (head) $ words $ camel $ replace_punct xs
