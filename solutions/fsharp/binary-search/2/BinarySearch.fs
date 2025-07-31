module BinarySearch
 
[<TailCall>]
let find input value =
    let rec binarySearch (input: 'T[]) value index op =
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
