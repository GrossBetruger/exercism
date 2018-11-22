module TwelveDays (recite) where

import Data.List

recite :: Int -> Int -> [String]
recite start stop = map (suffix) [start..stop]

suffix :: Int -> String
suffix n
 | n == 1 = base n ++ phrase 1 ++ "."
 | otherwise = base n ++ (intercalate ", " (map(phrase) (reverse [2..n]))) ++ ", and " ++ phrase 1 ++ "."

base n = "On the " ++ ordinal n ++ " day of Christmas my true love gave to me: "

phrase 1 = "a Partridge in a Pear Tree"
phrase 2 = "two Turtle Doves"
phrase 3 = "three French Hens"
phrase 4 = "four Calling Birds"
phrase 5 = "five Gold Rings"
phrase 6 = "six Geese-a-Laying"
phrase 7 = "seven Swans-a-Swimming"
phrase 8 = "eight Maids-a-Milking"
phrase 9 = "nine Ladies Dancing"
phrase 10 = "ten Lords-a-Leaping"
phrase 11 = "eleven Pipers Piping"
phrase 12 = "twelve Drummers Drumming"

ordinal 1 = "first"
ordinal 2 = "second"
ordinal 3 = "third"
ordinal 4 = "fourth"
ordinal 5 = "fifth"
ordinal 6 = "sixth"
ordinal 7 = "seventh"
ordinal 8 = "eighth"
ordinal 9 = "ninth"
ordinal 10 = "tenth"
ordinal 11 = "eleventh"
ordinal 12 = "twelfth"
