module SqueakyClean
open System

let transform (c: char) : string =
    match c with
    | '-' -> "_"
    | ' ' -> ""
    | d when Char.IsDigit d -> ""
    | u when Char.IsUpper u -> $"-{Char.ToLower u}"
    | g when Char.IsLower g
          && Char.IsBetween (g, char 0x0370, char 0x03FF) -> "?"
    | _ -> string c
    
let clean (identifier: string): string =
    identifier |> String.collect transform
