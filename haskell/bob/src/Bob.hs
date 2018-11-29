module Bob (responseFor) where

import Data.Char
import Data.List

responseFor :: String -> String
responseFor xs = (response . clearWhitespace) xs

response xs
  | null xs = "Fine. Be that way!"
  | isQuestionYell xs = "Calm down, I know what I'm doing!"
  | isQuestion xs = "Sure."
  | isYell xs = "Whoa, chill out!"
  | otherwise = "Whatever."

filterNonAlpha sentence = filter (\c -> isAlpha c) sentence

clearWhitespace sentence = filter (not . isSpace) sentence

isYell sentence
  | all (not . isAlpha) sentence = False
  | otherwise = all (\c -> isUpper c) (filterNonAlpha sentence)

isQuestion sentence = isSuffixOf "?" sentence

isQuestionYell sentence = isYell sentence && isQuestion sentence
