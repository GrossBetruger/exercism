module Clock (addDelta, fromHourMin, toString) where


data Clock = Clock {hour :: Int, minute:: Int} deriving (Show, Eq)


getHour :: Clock -> Int
getHour clock = hour clock

getMin :: Clock -> Int
getMin clock = minute clock


overflowMins minutes = minutes `mod` 60
overflowHours minutes = floor ((fromIntegral minutes) / 60)

rawHourToClockHour :: Clock -> Int
rawHourToClockHour clock = ((abs(getHour clock) + (overflowHours (getMin clock))) `mod` 24)
rawMinsToClockMins mins = (abs(mins) `mod` 60)

fromHourMin :: Int -> Int -> Clock
fromHourMin hour min = Clock hour min

showTime timeUnit
    | length (show(timeUnit)) == 1 = "0" ++ (show(timeUnit))
    | otherwise = show(timeUnit)


toString :: Clock -> String
toString clock =
    showTime (rawHourToClockHour clock) ++ ":" ++ showTime (rawMinsToClockMins(getMin clock))

addDelta :: Int -> Int -> Clock -> Clock
addDelta hour min clock = Clock (getHour clock + hour) (getMin clock + min)
