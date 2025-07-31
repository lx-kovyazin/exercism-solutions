module ValentinesDay

type Approval =
    | Yes
    | No
    | Maybe

type Cuisine =
    | Korean
    | Turkish

type Genre =
    | Crime
    | Horror
    | Romance
    | Thriller

type Activity =
    | BoardGame
    | Chill
    | Movie of Genre
    | Restaurant of Cuisine
    | Walk of int

let rateActivity (activity: Activity) : Approval =
    match activity with
    | BoardGame -> No
    | Chill -> No
    | Movie movie ->
        match movie with
        | Romance -> Yes
        | _ -> No
    | Restaurant restaurant ->
        match restaurant with
        | Korean -> Yes
        | Turkish -> Maybe
    | Walk walk ->
        match walk with
        | yes when yes < 3 -> Yes
        | mb when mb < 5 -> Maybe
        | _ -> No
