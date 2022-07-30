#[derive(Clone)]
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
        if as_bytes[index] == b'\n' {
            let line_vec = match index == as_bytes.len() {
                true => Vec::from(&as_bytes[start_index..]),
                false => Vec::from(&as_bytes[start_index..index]),
            };
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
        if as_bytes[i] == b';' || i == as_bytes.len() - 1 {
            let value = match i == as_bytes.len() - 1 {
                true => Vec::from(&raw_str[index..]),
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

fn update_team_team(team_stats: &mut TallyStats, match_state: MatchState) {
    team_stats.matches_played = team_stats.matches_played + 1;
    match match_state {
        MatchState::WIN => {
            team_stats.won = team_stats.won + 1;
            team_stats.points = team_stats.points + 3;
        }
        MatchState::LOSE => {
            team_stats.lost = team_stats.lost + 1;
        }
        MatchState::DRAW => {
            team_stats.drawn = team_stats.drawn + 1;
            team_stats.points = team_stats.points + 1;
        }
    }
}

fn team_two_state(team_one_state: MatchState) -> MatchState {
    match team_one_state {
        MatchState::DRAW => MatchState::DRAW,
        MatchState::LOSE => MatchState::WIN,
        MatchState::WIN => MatchState::LOSE,
    }
}

pub fn tally(match_results: &str) -> String {
    let lines = parse_lines(match_results);
    let mut teams: Vec<TallyStats> = Vec::new();

    for line in lines.iter() {
        let vals = parse_semi_color_separated_values(line.clone());

        let team_one_name = &vals[0];
        let team_two_name = &vals[1];
        let team_one_state = parse_match_result(&vals[2]).unwrap();

        match teams
            .iter_mut()
            .find(|team| team.team_name.eq(team_one_name))
        {
            Some(team) => {
                update_team_team(team, team_one_state.clone());
            }
            None => {
                let mut new_team_one = TallyStats {
                    team_name: String::clone(&team_one_name),
                    matches_played: 1,
                    won: 0,
                    drawn: 0,
                    points: 0,
                    lost: 0,
                };

                update_team_team(&mut new_team_one, team_one_state.clone());
                teams.push(new_team_one);
            }
        };

        match teams
            .iter_mut()
            .find(|team| team.team_name.eq(team_two_name))
        {
            Some(team) => {
                team.matches_played = team.matches_played + 1;

                match team_one_state {
                    MatchState::WIN => {
                        team.lost = team.lost + 1;
                    }
                    MatchState::LOSE => {
                        team.won = team.won + 1;
                        team.points = team.points + 3;
                    }
                    MatchState::DRAW => {
                        team.drawn = team.drawn + 1;
                        team.points = team.points + 1;
                    }
                }
            }
            None => {
                let mut new_team_two = TallyStats {
                    team_name: String::clone(&team_one_name),
                    matches_played: 1,
                    won: 0,
                    drawn: 0,
                    points: 0,
                    lost: 0,
                };

                update_team_team(&mut new_team_two, team_two_state(team_one_state.clone()));

                teams.push(new_team_two);
            }
        };
    }

    for team in teams.iter() {
        println!(
            "Team: {} Points: {} Played: {} Won: {} Lost: {} Drawn: {}",
            &team.team_name, &team.points, &team.matches_played, &team.won, &team.lost, &team.drawn
        );
    }

    unimplemented!(
        "Given the result of the played matches '{}' return a properly formatted tally table string.",
        match_results
    );
}
