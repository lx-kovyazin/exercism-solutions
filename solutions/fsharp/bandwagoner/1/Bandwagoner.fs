module Bandwagoner

type Coach = {
    Name        : string
    FormerPlayer: bool
}
type Stats = {
    Wins  : int
    Losses: int
}
type Team = {
    Name : string
    Coach: Coach
    Stats: Stats
}

let createCoach (name: string) (formerPlayer: bool): Coach = {
    Name         = name
    FormerPlayer = formerPlayer
}
let createStats(wins: int) (losses: int): Stats = {
    Wins   = wins
    Losses = losses 
}
let createTeam(name: string) (coach: Coach)(stats: Stats): Team = {
    Name  = name
    Coach = coach
    Stats = stats 
}
let replaceCoach(team: Team) (coach: Coach): Team = {
    team with Coach = coach
}
let isSameTeam(homeTeam: Team) (awayTeam: Team): bool =
    homeTeam = awayTeam
let rootForTeam(team: Team): bool =
    match team with
    | { Name = "Chicago Bulls" }
    | { Coach = { FormerPlayer = true } }
    | { Coach = { Name = "Gregg Popovich" } } -> true
    | { Stats = { Wins = wins ; Losses = losses } }
       when wins >= 60 || wins < losses -> true
    | _ -> false