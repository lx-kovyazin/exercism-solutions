class RemoteControlCar
{
    uint _distance = 0;
    uint _battery  = 100;

    bool IsBatteryDrained => _battery == 0;
    
    public static RemoteControlCar Buy() => new RemoteControlCar();

    public string DistanceDisplay() => $"Driven {_distance} meters";

    public string BatteryDisplay() => IsBatteryDrained ? "Battery empty" : $"Battery at {_battery}%";

    public void Drive()
    {
        if (IsBatteryDrained) return;
        
        _distance += 20;
        _battery  -= 1;
    }
}
