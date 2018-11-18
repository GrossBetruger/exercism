module LeapYear (isLeapYear) where


divisibleBy n m = rem n m == 0

isLeapYear :: Integer -> Bool
isLeapYear year
  | year `divisibleBy` 400 = True
  | year `divisibleBy` 4 && not (year `divisibleBy` 100) = True
  | otherwise = False