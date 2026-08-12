module DifferenceOfSquares

let private square number = pown number 2
let private sumBy mapper number = [1..number] |> Seq.sumBy mapper
let private fork j f g x = j (f x) (g x)

let squareOfSum (number: int): int = number |> (sumBy id >> square)
let sumOfSquares (number: int): int = number |> sumBy square
let differenceOfSquares (number: int): int = number |> fork (-) squareOfSum sumOfSquares