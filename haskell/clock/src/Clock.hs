module Clock (addDelta, fromHourMin, toString) where


data Clock = Clock {hour :: Int, minute:: Int} deriving (Show, Eq)

getHour :: Clock -> Int
getHour clock = hour clock

getMin :: Clock -> Int
getMin clock = minute clock

overflowMins minutes = minutes `mod` 60
overflowHours minutes = floor ((fromIntegral minutes) / 60)

rawTimeToClockHour :: Int -> Int -> Int
rawTimeToClockHour hours mins = ((hours + (overflowHours mins)) `mod` 24)

fromHourMin :: Int -> Int -> Clock
fromHourMin hour min = Clock (rawTimeToClockHour hour min) (overflowMins min)

showTime timeUnit
    | length (show(timeUnit)) == 1 = "0" ++ (show(timeUnit))
    | otherwise = show(timeUnit)


toString :: Clock -> String
toString clock =
    showTime (getHour clock) ++ ":" ++ showTime (getMin clock)

addDelta :: Int -> Int -> Clock -> Clock
addDelta hour min clock = fromHourMin (getHour clock + hour) (getMin clock + min)