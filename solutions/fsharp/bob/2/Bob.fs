module Bob
open System

let fork join left right arg = (left arg, right arg) ||> join
let response (input: string): string =
    let forall pred = fork (&&) (Seq.isEmpty >> not) (Seq.forall pred)
    let isYell(input: string): bool = input |> Seq.where Char.IsLetter |> forall Char.IsUpper
    let isQuestion(input: string): bool = input |> Seq.last |> (=) '?'
    let isYellQuestion = fork (&&) isYell isQuestion
    
    match input.Trim() with
    | "" -> "Fine. Be that way!"
    | yq when isYellQuestion yq -> "Calm down, I know what I'm doing!"
    | y when isYell y -> "Whoa, chill out!"
    | q when isQuestion q -> "Sure."
    | _ -> "Whatever."
