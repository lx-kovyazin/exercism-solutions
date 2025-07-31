using System;

class RemoteControlCar
{
    private readonly int speed;
    private readonly int batteryDrain;
    private int battery = 100;
    private int distance;

    public RemoteControlCar(int speed, int batteryDrain)
    {
        this.speed = speed;
        this.batteryDrain = batteryDrain;
    }

    public bool BatteryDrained() => battery < batteryDrain;

    public int DistanceDriven() => distance;

    public void Drive()
    {
        if (!BatteryDrained()) {
            distance += speed;
            battery  -= batteryDrain;
        }
    }

    public static RemoteControlCar Nitro() => new RemoteControlCar(50, 4);
}

class RaceTrack
{
    private readonly int distance;

    public RaceTrack(int distance) => this.distance = distance;

    public bool TryFinishTrack(RemoteControlCar car)
    {
        while (!car.BatteryDrained())
            car.Drive();
        return car.DistanceDriven() >= distance;
    }
}
