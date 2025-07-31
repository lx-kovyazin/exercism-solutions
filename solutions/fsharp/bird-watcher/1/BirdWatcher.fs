module BirdWatcher

open System

let lastWeek: int[] =
    [|0;2;5;3;7;8;4|]

let yesterday(counts: int[]): int =
    let yesterdayIndex = counts.Length - 2
    counts[yesterdayIndex]

let total(counts: int[]): int =
    Array.sum counts

let dayWithoutBirds(counts: int[]): bool =
    Array.exists (fun x -> x = 0) counts

let incrementTodaysCount(counts: int[]): int[] =
    let todayIndex = counts.Length - 1
    counts[todayIndex] <- counts[todayIndex] + 1
    counts

let unusualWeek(counts: int[]): bool =
    let intoDay (i, c) = (i + 1, c)
    let isEvenDay (d, _) = d % 2 = 0
    let isUnusualEven (_, c) = c = 0 || c = 10
    let isUnusualOdd (_, c) = c = 5
    
    match counts |> Array.indexed |> Array.map intoDay |> Array.partition isEvenDay with
    | (evens, _) when evens |> Array.forall isUnusualEven -> true
    | (_, odds) when odds |> Array.forall isUnusualOdd -> true
    | _ -> false
