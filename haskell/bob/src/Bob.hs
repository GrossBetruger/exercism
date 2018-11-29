module Bob (responseFor) where

import qualified Data.Text as T
import           Data.Text (Text)
import Data.Char
import Data.List


responseFor :: String -> Text
responseFor xs = (response . clearWhitespace) xs

response :: String -> Text
response xs
  | null xs = T.pack "Fine. Be that way!"
  | isQuestionYell xs = T.pack "Calm down, I know what I'm doing!"
  | isQuestion xs = T.pack "Sure."
  | isYell xs = T.pack "Whoa, chill out!"
  | otherwise = T.pack "Whatever."

filterNonAlpha sentence = filter (\c -> isAlpha c) sentence

clearWhitespace sentence = filter (not . isSpace) sentence

isYell sentence = any isUpper sentence && not (any isLower sentence)

isQuestion sentence = isSuffixOf "?" sentence

isQuestionYell sentence = isYell sentence && isQuestion sentence
