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

addToX (x, y) n = ((x + n), y)

addToY (x, y) n = (x, (y + n))

move :: Robot -> String -> Robot
move robot instructions
  | length instructions == 0 = robot
  | head instructions == 'R' && bearing robot == East = move (Robot South (coordinates robot)) (tail instructions)
  | head instructions == 'R' && bearing robot == South = move (Robot West (coordinates robot))  (tail instructions)
  | head instructions == 'R' && bearing robot == West = move (Robot North (coordinates robot))  (tail instructions)
  | head instructions == 'R' && bearing robot == North = move (Robot East (coordinates robot))  (tail instructions)
  | head instructions == 'L' && bearing robot == East = move (Robot North (coordinates robot))  (tail instructions)
  | head instructions == 'L' && bearing robot == South = move (Robot East (coordinates robot))  (tail instructions)
  | head instructions == 'L' && bearing robot == West = move (Robot South (coordinates robot))  (tail instructions)
  | head instructions == 'L' && bearing robot == North = move (Robot West (coordinates robot))  (tail instructions)
  | head instructions == 'A' && bearing robot == East = move (Robot East (addToX (coordinates robot) 1))  (tail instructions)
  | head instructions == 'A' && bearing robot == South = move (Robot South (addToY (coordinates robot) (-1)))  (tail instructions)
  | head instructions == 'A' && bearing robot == West = move (Robot West (addToX (coordinates robot) (-1)))  (tail instructions)
  | head instructions == 'A' && bearing robot == North = move (Robot North (addToY (coordinates robot) 1))  (tail instructions)
