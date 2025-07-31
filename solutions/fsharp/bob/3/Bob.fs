module Bob
open System

let inline (=>) fork join arg = fork |> Seq.map (fun f -> f arg) |> Seq.reduce join
let response (input: string): string =
    let forall pred = [not << Seq.isEmpty; Seq.forall pred] => (&&)
    let isYell(input: string): bool = input |> Seq.where Char.IsLetter |> forall Char.IsUpper
    let isQuestion(input: string): bool = input |> Seq.last |> (=) '?'
    let isYellQuestion = [isYell; isQuestion] => (&&)
    
    match input.Trim() with
    | "" -> "Fine. Be that way!"
    | yq when isYellQuestion yq -> "Calm down, I know what I'm doing!"
    | y when isYell y -> "Whoa, chill out!"
    | q when isQuestion q -> "Sure."
    | _ -> "Whatever."
