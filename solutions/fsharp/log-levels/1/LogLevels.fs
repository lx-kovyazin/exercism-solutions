module LogLevels

let split (logLine: string) = 
    match logLine.Split ':' with
    | [|level; message|] -> (level.Trim([|'['; ']'|]).ToLower(), message.Trim())
    | _ -> failwith $"It's not a {nameof logLine} = \"{logLine}\"."

let message (logLine: string): string =
    let (_, message) = split logLine
    message

let logLevel(logLine: string): string =
    let (level, _) = split logLine
    level

let reformat(logLine: string): string =
    let (level, message) = split logLine
    $"{message} ({level})"
