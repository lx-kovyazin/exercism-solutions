module DifferenceOfSquares

let private genSeq number = [1..number]
let private square number = pown number 2

let squareOfSum (number: int): int = number |> genSeq |> Seq.sum |> square
let sumOfSquares (number: int): int = number |> genSeq |> Seq.sumBy square
let differenceOfSquares (number: int): int = squareOfSum number - sumOfSquares number