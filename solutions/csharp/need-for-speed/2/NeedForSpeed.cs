class RemoteControlCar
{
    readonly int speed;
    readonly int batteryDrain;
    int battery = 100;
    int distance;

    public RemoteControlCar(int speed, int batteryDrain)
    {
        this.speed = speed;
        this.batteryDrain = batteryDrain;
    }

    public static RemoteControlCar Nitro() => new(50, 4);
    
    public bool BatteryDrained() => battery < batteryDrain;
    
    public int DistanceDriven() => distance;
    
    public void Drive()
    {
        if (BatteryDrained()) return;
        
        distance += speed;
        battery  -= batteryDrain;
    }

}

class RaceTrack
{
    readonly int distance;

    public RaceTrack(int distance) => this.distance = distance;

    public bool TryFinishTrack(RemoteControlCar car)
    {
        while (!car.BatteryDrained())
            car.Drive();
        return car.DistanceDriven() >= distance;
    }
}
