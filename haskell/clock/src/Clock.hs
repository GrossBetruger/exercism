module Clock (addDelta, fromHourMin, toString) where


data Clock = Clock {hour :: Int, minute:: Int} deriving (Show, Eq)


getHour :: Clock -> Int
getHour clock = hour clock

getMin :: Clock -> Int
getMin clock = minute clock


fromHourMin :: Int -> Int -> Clock
fromHourMin hour min = Clock hour min

showTime timeUnit
    | length (show(timeUnit)) == 1 = "0" ++ (show(timeUnit))
    | otherwise = show(timeUnit)


toString :: Clock -> String
toString clock = showTime (abs (mod (getHour clock) 24)) ++ ":" ++ showTime (abs (mod (getMin clock) 60))

addDelta :: Int -> Int -> Clock -> Clock
addDelta hour min clock = error "You need to implement this function."
