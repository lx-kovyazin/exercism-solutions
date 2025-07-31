class Lasagna
{
    public int ExpectedMinutesInOven() => 40;
    
    public int RemainingMinutesInOven(int actualMinutesInOven)
        => ExpectedMinutesInOven() - actualMinutesInOven;
    
    public int PreparationTimeInMinutes(int numberOfLayers) => 2 * numberOfLayers;
    
    public int ElapsedTimeInMinutes(int numberOfLayers, int actualMinutesInOven)
        => PreparationTimeInMinutes(numberOfLayers) + actualMinutesInOven;
}
