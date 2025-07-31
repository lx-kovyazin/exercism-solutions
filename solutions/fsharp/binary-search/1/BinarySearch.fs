module BinarySearch
 
let find (input: 'T array) (value: 'T): option<int> =
    let rec binarySearch (input: 'T array) (value:'T) (index: int) op: option<int> =
        match Seq.length input with
        | 0 -> None
        | length ->
            let mid = (length - 1) / 2
            let index = [mid; 1] |> Seq.fold op index
            match input[mid] with
            | gt when gt > value -> binarySearch input[..mid - 1] value index (-)
            | lt when lt < value -> binarySearch input[mid + 1..] value index (+)
            | _ -> Some(index)
    binarySearch input value -1 (+)
