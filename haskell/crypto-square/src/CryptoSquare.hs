module CryptoSquare (encode) where

import Data.Char

edgeSize :: String -> Int
edgeSize plain = (floor (sqrt (fromIntegral (length plain))))

toGrid xs dimension
  | xs == [] = []
  | otherwise = [take dimension xs] ++ (toGrid (drop dimension xs) dimension)

transpose vec
  | sum (map length vec) == 0 = []
  | otherwise = [(map head vec)] ++ transpose (map tail vec)

lower s = map toLower s

encode :: String -> String
encode xs = concat $ transpose $ (toGrid (lower (filter (\c -> not (elem c "@1,%!. \n\t")) xs)) (edgeSize xs))
