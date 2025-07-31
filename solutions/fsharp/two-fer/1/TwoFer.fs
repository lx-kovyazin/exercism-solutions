module TwoFer

let template(name: string): string =
   $"One for {name}, one for me."
let twoFer (input: string option): string =
  match input with
  | None -> "you"
  | Some name -> name
  |> template