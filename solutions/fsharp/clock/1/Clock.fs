module Clock

/// Euclidean modulo
let private (mod) dividend divisor =
    match dividend % divisor with
    | r when r < 0 -> r + divisor
    | r -> r

let private minutesInHour = 60
let private intoMinutes hours = minutesInHour * hours
let private modDay m = 24 |> intoMinutes |> (mod) m

let add minutes clock = clock + minutes |> modDay
let subtract minutes clock = clock - minutes |> modDay
let create hours minutes = hours |> intoMinutes |> add minutes
let display clock = clock |> fun c -> (c / minutesInHour, c % minutesInHour) ||> sprintf "%02i:%02i"