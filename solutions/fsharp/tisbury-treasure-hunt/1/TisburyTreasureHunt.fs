module TisburyTreasureHunt

let getCoordinate (line: string * string): string = snd line

let convertCoordinate (coordinate: string): int * char =
    match coordinate.ToCharArray() with
    | [| d ; l |] -> (d |> string |> int, l)
    | _ -> failwith "Bad input coordinate format"
    
let compareRecords (azarasData: string * string) (ruisData: string * (int * char) * string) : bool = 
    let _, coordinates, _ = ruisData
    azarasData |> getCoordinate |> convertCoordinate = coordinates
    
let createRecord (azarasData: string * string) (ruisData: string * (int * char) * string) : string * string * string * string =
   match azarasData, ruisData with
   | (treasure, coordinates), (location, _, quandrant)
       when compareRecords azarasData ruisData
       -> coordinates, location, quandrant, treasure
   | _ -> "", "", "", ""