module Phone (number) where

import Data.Char

filterInternationalCountryCode number
  | take 1 number == "1" = drop 1 number
  | take 2 number == "+1" = drop 2 number
  | otherwise = number

filterSymbols number = filter (isDigit) number

validate11Digits number
  | take 1 number == "1" = True
  | take 2 number == "+1" = True
  | otherwise = False

validSubscriberNumber number
  | elem (take 1 (filterSymbols $ filterInternationalCountryCode number)) ["0","1"] = False
  | elem (take 1 (drop 3 (filterSymbols $ filterInternationalCountryCode number))) ["0","1"] = False
  | otherwise = True

number :: String -> Maybe String
number xs
 | (length (filterSymbols xs)) < 10 = Nothing
 | validSubscriberNumber xs == False = Nothing
 | length (filterSymbols xs) == 10  = Just $ filterSymbols $ filterInternationalCountryCode xs
 | validate11Digits xs == True = Just $ filterSymbols $ filterInternationalCountryCode xs
 | otherwise = Nothing
 