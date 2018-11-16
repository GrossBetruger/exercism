module CryptoSquare (encode) where

import Data.Char
import Data.List

edgeSize :: String -> Int
edgeSize plain = (floor (sqrt (fromIntegral (length plain))))

toGrid xs dimension
  | xs == [] = []
  | otherwise = [take dimension xs] ++ (toGrid (drop dimension xs) dimension)

transposeGrid vec
  | sum (map length vec) == 0 = []
  | otherwise = [(map head vec)] ++ transpose (map tail vec)

lower s = map toLower s

joinSpaced s = intercalate " " s

clean s = filter (\c -> not (elem c "@,%!. \n\t")) s

encode :: String -> String
encode xs = joinSpaced $ transposeGrid $ (toGrid (lower $ clean xs) (edgeSize xs))
