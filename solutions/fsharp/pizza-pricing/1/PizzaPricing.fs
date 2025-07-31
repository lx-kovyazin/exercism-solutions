module PizzaPricing

type Pizza =
    | Margherita
    | Caprese
    | Formaggio
    | ExtraSauce of Pizza
    | ExtraToppings of Pizza
    
let rec pizzaPrice (pizza: Pizza): int =
    match pizza with
    | Margherita -> 7
    | Caprese -> 9
    | Formaggio -> 10
    | ExtraSauce pizza -> pizzaPrice pizza + 1
    | ExtraToppings pizza -> pizzaPrice pizza + 2

let orderPrice(pizzas: Pizza list): int =
    let price = pizzas |> Seq.sumBy pizzaPrice
    let fee = match pizzas.Length with | 1 -> 3 | 2 -> 2 | _ -> 0
    price + fee