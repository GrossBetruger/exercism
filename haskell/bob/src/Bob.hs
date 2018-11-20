module Bob (responseFor) where

import Data.Char

responseFor :: String -> String
responseFor xs
  | isQuestYell xs = "Calm down, I know what I'm doing!"
  | isQuestion xs = "Sure."
  | isYell xs = "Whoa, chill out!"
  | xs == "" = "Fine. Be that way!"
  | otherwise = "Whatever."


filterNonAlpha sentence = filter (\c -> isAlpha c) sentence

allCaps sentence = all (\c -> isUpper c) (filterNonAlpha sentence)

isYell sentence = allCaps sentence

isQuestion sentence = elem '?' sentence

isQuestYell sentence = isYell sentence && isQuestion sentence
