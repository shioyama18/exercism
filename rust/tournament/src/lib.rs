use std::collections::HashMap;

struct Standing {
    name: String,
    wins: u32,
    losses: u32,
    ties: u32,
    games: u32,
    points: u32,
}

impl Standing {
    fn new(name: &str) -> Self {
        Standing {
            name: name.to_string(),
            wins: 0,
            losses: 0,
            ties: 0,
            games: 0,
            points: 0,
        }
    }
}

enum GameResult {
    Win,
    Draw,
    Loss,
}

pub fn tally(match_results: &str) -> String {
    let mut result = HashMap::new();

    for line in match_results.lines() {
        let game = line.split(';').collect::<Vec<_>>();
        let team1 = game[0];
        let team2 = game[1];
        match game[2] {
            "win" => {
                update_team(&mut result, team1, GameResult::Win);
                update_team(&mut result, team2, GameResult::Loss);
            }
            "loss" => {
                update_team(&mut result, team1, GameResult::Loss);
                update_team(&mut result, team2, GameResult::Win);
            }
            "draw" => {
                update_team(&mut result, team1, GameResult::Draw);
                update_team(&mut result, team2, GameResult::Draw);
            }
            _ => panic!("Unexpected input")
        }
    }

    let mut tally = result.values().collect::<Vec<_>>();
    tally.sort_by(|a, b| {
        if a.points == b.points {
            a.name.cmp(&b.name)
        }
        else {
            b.points.cmp(&a.points)
        }
    });

    let mut output = table_line("Team", "MP", "W", "D", "L", "P");
    for team in tally {
        output = output + "\n" +
            &table_line(&team.name,
                       &team.games.to_string(),
                       &team.wins.to_string(),
                       &team.ties.to_string(),
                       &team.losses.to_string(),
                       &team.points.to_string());
    }

    output    
}

fn table_line(team: &str, mp: &str, w: &str, d: &str, l: &str, points: &str) -> String {
    format!("{:31}| {:>2} | {:>2} | {:>2} | {:>2} | {:>2}", team, mp, w, d, l, points)
}


fn update_team(result: &mut HashMap<String, Standing>, team: &str, game_result: GameResult) {
    let entry = result.entry(team.to_string()).or_insert(Standing::new(team));
    match game_result {
        GameResult::Win => {
            (*entry).wins += 1;
            (*entry).games += 1;
            (*entry).points += 3;
        }
        GameResult::Draw => {
            (*entry).ties += 1;
            (*entry).games += 1;
            (*entry).points += 1;
        }
        GameResult::Loss => {
            (*entry).losses += 1;
            (*entry).games += 1;
        }
    }
}
