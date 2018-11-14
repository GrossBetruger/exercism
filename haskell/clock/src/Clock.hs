module Clock (addDelta, fromHourMin, toString) where

--data Hour = Int
--data Min = Int
data Clock = Clock {hour :: Int, min:: Int} deriving (Show, Eq)

fromHourMin :: Int -> Int -> Clock
fromHourMin hour min = Clock hour min

toString :: Clock -> String
toString clock = error "You need to implement this function."

addDelta :: Int -> Int -> Clock -> Clock
addDelta hour min clock = error "You need to implement this function."
