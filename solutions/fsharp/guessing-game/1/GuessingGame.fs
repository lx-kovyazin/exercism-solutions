module GuessingGame

let reply (guess: int): string = 
    match guess with
    | 42 -> "Correct"
    | 41 | 43 -> "So close"
    | h when h > 43 -> "Too high"
    | _ -> "Too low"
