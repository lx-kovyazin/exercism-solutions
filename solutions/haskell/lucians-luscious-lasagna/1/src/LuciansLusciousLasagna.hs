module LuciansLusciousLasagna (elapsedTimeInMinutes, expectedMinutesInOven, preparationTimeInMinutes) where

expectedMinutesInOven :: Integer
expectedMinutesInOven = 40

preparationTimeInMinutes :: Num a => a -> a
preparationTimeInMinutes numOfLayers = numOfLayers * 2

elapsedTimeInMinutes :: Num a => a -> a -> a
elapsedTimeInMinutes numOfLayers actualMinutesInOven =
  preparationTimeInMinutes numOfLayers + actualMinutesInOven