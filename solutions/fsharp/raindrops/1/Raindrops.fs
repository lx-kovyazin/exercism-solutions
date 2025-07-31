module Raindrops

let inline noneIfEmpty (s: string): string option =
    if s.Length = 0 then None else Some s

let convert (number: int): string = 
    [|(3, "Pling"); (5, "Plang"); (7, "Plong")|]
    |> Array.filter (fun (m, _) -> number % m = 0)
    |> Array.map (fun (_, r) -> r)
    |> String.concat ""
    |> noneIfEmpty
    |> (Option.defaultValue << string) number
        