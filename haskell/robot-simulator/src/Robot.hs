module Robot
    ( Bearing(East,North,South,West)
    , bearing
    , coordinates
    , mkRobot
    , move
    ) where

data Bearing = North
             | East
             | South
             | West
             deriving (Eq, Show)

data Robot = Robot {direction :: Bearing, coords :: (Integer, Integer)}

bearing :: Robot -> Bearing
bearing robot = direction robot

coordinates :: Robot -> (Integer, Integer)
coordinates robot = coords robot

mkRobot :: Bearing -> (Integer, Integer) -> Robot
mkRobot direction coordinates = Robot direction coordinates

move :: Robot -> String -> Robot
move robot instructions
  | instructions == "turn right" && bearing robot == East = Robot West (coordinates robot)
  | otherwise = robot
