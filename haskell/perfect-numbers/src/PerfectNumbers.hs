module PerfectNumbers (classify, Classification(..)) where


import Data.Maybe


data Classification = Deficient | Perfect | Abundant deriving (Eq, Show)

dividedBy n m = rem n m == 0

factors n = filter (\x -> n `dividedBy` x) [1..n-1]

aliquotSum n = sum (factors n)

classify :: Int -> Maybe Classification
classify n
  | n < 1 = Nothing
  | aliquotSum n > n = Just Abundant
  | aliquotSum n < n = Just Deficient
  | aliquotSum n == n = Just Perfect
