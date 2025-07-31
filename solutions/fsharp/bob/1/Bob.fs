module Bob
open System

let isQuestion(input: string): bool =
    input |> Seq.last |> (=) '?'
let isYell(input: string): bool =
    match input |> Seq.where Char.IsLetter |> Seq.toList with
    | [] -> false
    | letters -> letters |> Seq.forall Char.IsUpper
let response (input: string): string =
    match input.Trim() with
    | silence when String.IsNullOrWhiteSpace silence -> "Fine. Be that way!"
    | yq when isYell yq && isQuestion yq -> "Calm down, I know what I'm doing!"
    | q when isQuestion q -> "Sure."
    | y when isYell y -> "Whoa, chill out!"
    | _ -> "Whatever."