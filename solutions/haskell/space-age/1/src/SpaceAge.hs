module SpaceAge (Planet(..), ageOn) where

data Planet = Mercury
            | Venus
            | Earth
            | Mars
            | Jupiter
            | Saturn
            | Uranus
            | Neptune

orbitalPeriod :: Planet -> Float
orbitalPeriod planet = case planet of
    Mercury -> 000.24084670
    Venus   -> 000.61519726
    Earth   -> 001.00000000
    Mars    -> 001.88081580
    Jupiter -> 011.86261500
    Saturn  -> 029.44749800
    Uranus  -> 084.01684600
    Neptune -> 164.79132000


ageOn :: Planet -> Float -> Float
ageOn planet seconds = seconds / (orbitalPeriod planet * 31557600)
