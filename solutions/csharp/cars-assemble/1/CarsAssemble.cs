using System;

static class AssemblyLine
{
    public static double SuccessRate(int speed) => speed switch
    {
        0   => 0,
        < 5 => 1,
        < 9 => 0.9,
        9   => 0.8,
        10  => 0.77,
        _   => throw new InvalidOperationException("An invalid speed value."),
    };
    
    public static double ProductionRatePerHour(int speed) => 221.0 * speed * SuccessRate(speed);

    public static int WorkingItemsPerMinute(int speed) => (int)(ProductionRatePerHour(speed) / 60);
}
