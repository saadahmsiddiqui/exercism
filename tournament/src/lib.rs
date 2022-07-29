use std::cmp::Ordering;

enum MatchState {
    WIN,
    LOSE,
    DRAW,
}
struct TallyStats {
    team_name: String,
    matches_played: u16,
    won: u16,
    drawn: u16,
    lost: u16,
    points: u16,
}

pub fn parse_lines(match_results: &str) -> Vec<String> {
    let mut all_lines: Vec<String> = Vec::new();

    let as_bytes = match_results.as_bytes();

    let mut start_index: usize = 0;

    for index in 0..as_bytes.len() {
        if as_bytes[index] == b'\n' || index + 1 == as_bytes.len() {
            let line_vec = Vec::from(&as_bytes[start_index..index + 1]);
            let line = String::from_utf8(line_vec).unwrap();

            all_lines.push(line);
            start_index = index + 1;
        }
    }

    return all_lines;
}

pub fn parse_semi_color_separated_values(raw_str: String) -> Vec<String> {
    let mut values_vec: Vec<String> = Vec::new();
    let as_bytes = raw_str.as_bytes();

    let mut index: usize = 0;

    for i in 0..as_bytes.len() {
        if as_bytes[i] == b';' || i + 1 == as_bytes.len() {
            let value = match i + 1 == as_bytes.len() {
                true => Vec::from(&raw_str[index..i]),
                false => Vec::from(&raw_str[index..i]),
            };
            let utf8_str = String::from_utf8(value).unwrap();
            values_vec.push(utf8_str);
            index = i + 1;
        }
    }

    return values_vec;
}

fn parse_match_result(match_result: &String) -> Option<MatchState> {
    println!("{} {}", match_result, match_result.eq("win"));

    if match_result.eq(&String::from("win")) {
        return Some(MatchState::WIN);
    }
    
    if match_result.eq(&String::from("loss")) {
        return Some(MatchState::LOSE);
    }

    if match_result.eq(&String::from("draw")) {
        return Some(MatchState::DRAW);
    }

    return None;
}

pub fn tally(match_results: &str) -> String {
    let lines = parse_lines(match_results);
    let mut teams: Vec<TallyStats> = Vec::new();

    for line in lines.iter() {
        let vals = parse_semi_color_separated_values(line.clone());

        let team_one_name = &vals[0];
        let team_two_name = &vals[1];
        let team_one_state = parse_match_result(&vals[2]).unwrap();

        match teams.iter_mut().find(|team| {
            team.team_name.cmp(team_one_name) == Ordering::Equal
                || team.team_name.cmp(team_two_name) == Ordering::Equal
        }) {
            Some(team) => {
                team.matches_played = team.matches_played + 1;

                if team_one_name.cmp(&team.team_name) == Ordering::Equal {
                    match team_one_state {
                        MatchState::WIN => {
                            team.won = team.won + 1;
                            team.points = team.points + 3;
                        }
                        MatchState::LOSE => {
                            team.lost = team.lost + 1;
                        }
                        MatchState::DRAW => {
                            team.drawn = team.drawn + 1;
                        }
                    }
                }
                if team_two_name.cmp(&team.team_name) == Ordering::Equal {
                    match team_one_state {
                        MatchState::WIN => {
                            team.lost = team.lost + 1;
                        }
                        MatchState::LOSE => {
                            team.points = team.points + 3;
                            team.won = team.won + 1;
                        }
                        MatchState::DRAW => {
                            team.drawn = team.drawn + 1;
                        }
                    }
                }
            }
            None => {}
        };
    }


    for team in teams.iter() {
        println!("Team: {} Points: {} Played: {} Won: {} Lost: {} Drawn: {}", &team.team_name, &team.points, &team.matches_played, &team.won, &team.lost, &team.drawn);
    }

    unimplemented!(
        "Given the result of the played matches '{}' return a properly formatted tally table string.",
        match_results
    );
}
