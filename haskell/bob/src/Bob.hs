module Bob (responseFor) where

import Data.Char

responseFor :: String -> String
responseFor xs
  | isQuestYell xs = "Calm down, I know what I'm doing!"
  | isQuestion xs = "Sure."
  | isYell xs = "Whoa, chill out!"
  | (clearWhitespace xs) == "" = "Fine. Be that way!"
  | otherwise = "Whatever."

filterNonAlpha sentence = filter (\c -> isAlpha c) sentence

clearWhitespace sentence = filter (\c -> not (elem c " \n\t\r")) sentence

isYell sentence
  | length (filterNonAlpha sentence) == 0 = False
  | otherwise = all (\c -> isUpper c) (filterNonAlpha sentence)

isQuestion sentence
  | length (clearWhitespace sentence) == 0 = False
  | otherwise = last (clearWhitespace sentence) == '?'

isQuestYell sentence = isYell sentence && isQuestion sentence
