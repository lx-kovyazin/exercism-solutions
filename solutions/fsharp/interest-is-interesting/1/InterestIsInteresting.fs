module InterestIsInteresting

open System
let private apply value percent = value * percent / 100m
let interestRate (balance: decimal): single =
    match balance with
    | neg  when neg  < 0m    -> 3.213f
    | pos1 when pos1 < 1000m -> 0.5f
    | pos2 when pos2 < 5000m -> 1.621f
    | _                      -> 2.475f
let interest (balance: decimal): decimal =
    balance |> interestRate |> decimal |> apply balance
let annualBalanceUpdate(balance: decimal): decimal =
    balance |> interest |> (+) balance
let amountToDonate(balance: decimal) (taxFreePercentage: float): int =
    if Decimal.IsPositive balance then taxFreePercentage * 2. else 0.
    |> decimal |> apply balance |> int