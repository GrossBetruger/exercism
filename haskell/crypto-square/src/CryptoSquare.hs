module CryptoSquare (encode) where

edgeSize :: String -> Int
edgeSize plain = (floor (sqrt (fromIntegral (length plain))))

toGrid xs dimension
  | xs == [] = []
  | otherwise = [take dimension xs] ++ (toGrid (drop dimension xs) dimension)

encode :: String -> String
encode xs = filter (\c -> not (elem c "@1,%!. \n\t")) xs
