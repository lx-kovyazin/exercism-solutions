module CarsAssemble

let successRate (speed: int) : float =
    match speed with
    | 0 -> 0.0
    | s when s <= 4 -> 1.0
    | s when s <= 8 -> 0.9
    | 9 -> 0.8
    | 10 -> 0.77
    | _ -> failwith $"Unexpected: {nameof speed}={speed}"

let productionRatePerHour (speed: int) : float =
    221.0 * float speed * successRate (speed)

let workingItemsPerMinute (speed: int) : int =
    int (productionRatePerHour speed / 60.0)
