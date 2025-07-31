module Allergies

type Allergen =
    | Eggs         = 0x01
    | Peanuts      = 0x02
    | Shellfish    = 0x04
    | Strawberries = 0x08
    | Tomatoes     = 0x10
    | Chocolate    = 0x20
    | Pollen       = 0x40
    | Cats         = 0x80

let allergicTo (codedAllergies:int) (allergen: Allergen) =
    codedAllergies &&& int allergen <> 0

let list codedAllergies =
    Allergen.GetValues()
    |> Array.filter (fun a -> allergicTo codedAllergies a)
    |> Array.toList
