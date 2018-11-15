module Clock (addDelta, fromHourMin, toString) where


data Clock = Clock {hour :: Int, minute:: Int} deriving (Show, Eq)

getHour :: Clock -> Int
getHour clock = hour clock

getMin :: Clock -> Int
getMin clock = minute clock


fromHourMin :: Int -> Int -> Clock
fromHourMin hour min = Clock hour min

toString :: Clock -> String
toString clock = show (getHour clock) ++ ":" ++ show (getMin clock)

addDelta :: Int -> Int -> Clock -> Clock
addDelta hour min clock = error "You need to implement this function."
