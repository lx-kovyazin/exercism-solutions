module CollatzConjecture

let rec steps (number: int): int option = 
    match number with
    | i when i <= 0 -> None
    | 1 -> Some 0
    | n -> match n with
           | x when x % 2 = 0 -> x / 2
           | x -> 3 * x + 1
           |> steps
           |> Option.map (fun step -> step + 1)