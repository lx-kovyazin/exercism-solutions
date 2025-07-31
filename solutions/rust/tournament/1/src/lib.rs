use std::{collections::HashMap, ops::Not};

#[derive(Clone, Copy, PartialEq)]
enum MatchResult {
    Loss = 0,
    Draw = 1,
    Win  = 3,
}

impl Not for MatchResult {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            MatchResult::Loss => MatchResult::Win,
            MatchResult::Draw => MatchResult::Draw,
            MatchResult::Win  => MatchResult::Loss,
        }
    }
}

impl MatchResult {
    fn match_result(src: &str) -> Option<Self> {
        match src {
            "loss" => Some(Self::Loss),
            "draw" => Some(Self::Draw),
            "win"  => Some(Self::Win),
            _      => None,
        }
    }

    fn count(&self, results: &Vec<&MatchResult>) -> usize {
        results.iter().filter(|&&r| r.eq(self)).count()
    }

    fn points(results: &Vec<&MatchResult>) -> usize {
        results.iter().fold(0, |points, &&result| points + result as usize)
    }

    fn scores(match_results: Vec<&MatchResult>) -> Scores {
        const SCORE_RULES: [fn(&Vec<&MatchResult>) -> usize; 5] = [
            |rs| rs.len(),
            |rs| MatchResult::Win.count(rs),
            |rs| MatchResult::Draw.count(rs),
            |rs| MatchResult::Loss.count(rs),
            |rs| MatchResult::points(rs),
        ];
        SCORE_RULES.map(|rule| rule(&match_results))
    }
}

type Team = String;
type Matches = HashMap<Team, Vec<MatchResult>>;
type Tournament = HashMap<Team, Matches>;
type Scores = [usize; 5];

pub fn tally(match_results: &str) -> String {
    let mut table: Vec<(Team, Scores)> = match_results
        .split('\n')
        .map(|match_info| match_info.split(';').collect::<Vec<&str>>())
        .filter(|match_info| match_info.len() == 3)
        .fold(Tournament::new(), |mut tournament, match_info|{
            let team_1 = Team::from(match_info[0]);
            let team_2 = Team::from(match_info[1]);
            let result = MatchResult::match_result(match_info[2]).unwrap();
            tournament.entry(team_1.clone()).or_insert(Matches::new()).entry(team_2.clone()).or_insert(Vec::new()).push(result);
            tournament.entry(team_2).or_insert(Matches::new()).entry(team_1).or_insert(Vec::new()).push(!result);
            tournament
        })
        .into_iter()
        .map(|(team, matches)| {
            (team, MatchResult::scores(matches.values().flat_map(|vs| vs.iter()).collect()))
        })
        .collect();
    table.sort_by(|a, b| a.0.cmp(&b.0));
    table.sort_by(|a, b| b.1[4].cmp(&a.1[4]));

    let create_row = |team, scores: [String; 5]| format!("{team:<30}{}", scores.map(|s| format!(" | {s:>2}")).concat());
    [create_row("Team", ["MP", "W", "D", "L", "P"].map(|s| s.to_string()))]
    .into_iter()
    .chain(
        table.iter().map(|row| create_row(&row.0, row.1.map(|s| s.to_string())))
    )
    .collect::<Vec<_>>()
    .join("\n")
}
