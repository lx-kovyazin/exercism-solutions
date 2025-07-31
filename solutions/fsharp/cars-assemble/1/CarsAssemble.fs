module CarsAssemble

let successRate (speed: int) : float =
    match speed with
    | s when 1 <= s && s <= 4 -> 1.0
    | s when 5 <= s && s <= 8 -> 0.9
    | 9 -> 0.8
    | 10 -> 0.77
    | _ -> 0

let productionRatePerHour (speed: int) : float =
    221.0 * float speed * successRate (speed)

let workingItemsPerMinute (speed: int) : int =
    int (productionRatePerHour speed / 60.0)
