module SpaceAge (Planet(..), ageOn) where

data Planet = Mercury
            | Venus
            | Earth
            | Mars
            | Jupiter
            | Saturn
            | Uranus
            | Neptune deriving(Ord, Eq)

earthYearSeconds :: Float
earthYearSeconds = 31557600

toEarthSeconds :: Planet -> Float
toEarthSeconds Earth = 1
toEarthSeconds Mercury = 0.2408467
toEarthSeconds Venus = 0.61519726
toEarthSeconds Mars = 1.8808158
toEarthSeconds Jupiter = 11.862615
toEarthSeconds Saturn = 29.447498
toEarthSeconds Uranus = 84.016846
toEarthSeconds Neptune = 164.79132

ageOn :: Planet -> Float -> Float
ageOn planet seconds = seconds / (earthYearSeconds * (toEarthSeconds planet))
