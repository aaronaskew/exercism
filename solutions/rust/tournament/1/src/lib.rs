use std::collections::HashMap;
use MatchResult::*;

pub fn tally(match_results: &str) -> String {
    let results: Vec<Vec<&str>> = match_results
        .lines()
        .map(|l| l.split(';').collect())
        .collect();

    score(results)
}

fn score(results: Vec<Vec<&str>>) -> String {
    let mut score = "Team                           | MP |  W |  D |  L |  P".to_string();
    let mut stats: HashMap<String, Stats> = HashMap::new();

    for result in results {
        let (team1, team2, result) = (
            result[0].to_string(),
            result[1].to_string(),
            match result[2] {
                "win" => Win,
                "loss" => Loss,
                _ => Draw,
            },
        );
        // team1 stats
        stats
            .entry(team1)
            .and_modify(|stats| *stats = stats.add_match(&result))
            .or_insert(Stats::new(&result));

        // team2 stats
        let result2 = match result {
            Win => Loss,
            Loss => Win,
            Draw => Draw,
        };
        stats
            .entry(team2)
            .and_modify(|stats| *stats = stats.add_match(&result2))
            .or_insert(Stats::new(&result2));
    }

    let mut points_team: Vec<(u32, &String)> = stats
        .iter()
        .map(|(team, stats)| (stats.points(), team))
        .collect();

    points_team.sort_unstable_by(|a, b| {
        if a.0 == b.0 {
            a.1.partial_cmp(b.1).unwrap()
        } else {
            b.0.partial_cmp(&a.0).unwrap()
        }
    });

    points_team.iter().for_each(|(points, team)| {
        let stats = stats.get(*team).unwrap();
        score += &format!(
            "\n{:<31}| {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
            team,
            stats.mp(),
            stats.w,
            stats.d,
            stats.l,
            points
        );
    });

    score
}

#[derive(Eq, PartialEq)]
enum MatchResult {
    Win,
    Loss,
    Draw,
}

#[derive(Debug)]
struct Stats {
    w: u32,
    d: u32,
    l: u32,
}

impl Stats {
    fn new(result: &MatchResult) -> Self {
        Self {
            w: if *result == Win { 1 } else { 0 },
            d: if *result == Draw { 1 } else { 0 },
            l: if *result == Loss { 1 } else { 0 },
        }
    }

    fn add_match(&self, result: &MatchResult) -> Self {
        Self {
            w: self.w + if *result == Win { 1 } else { 0 },
            d: self.d + if *result == Draw { 1 } else { 0 },
            l: self.l + if *result == Loss { 1 } else { 0 },
        }
    }

    fn mp(&self) -> u32 {
        self.w + self.l + self.d
    }

    fn points(&self) -> u32 {
        self.w * 3 + self.d
    }
}
