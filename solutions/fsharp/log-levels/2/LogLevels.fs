module LogLevels

let split (logLine: string): string * string = 
    match logLine.Split ':' with
    | [|level; message|] -> (level.Trim([|'['; ']'|]).ToLower(), message.Trim())
    | _ -> failwith $"Invalid argument: {nameof logLine} = \"{logLine}\"."

let message (logLine: string): string =
    split logLine |> snd

let logLevel(logLine: string): string =
    split logLine |> fst

let reformat(logLine: string): string =
    let level, message = split logLine
    $"{message} ({level})"
