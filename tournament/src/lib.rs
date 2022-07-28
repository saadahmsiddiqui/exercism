// enum MatchState {
//     WIN,
//     LOSE,
//     DRAW
// }

// struct TeamPerformance {
//     team_name: String,
//     matches_played: String,
//     won: u16,
//     drawn: u16,
//     lost: u16,
//     points: u16,
// }

pub fn parse_lines(match_results: &str) -> Vec<String> {
    let mut all_lines: Vec<String> = Vec::new();
    
    let as_bytes = match_results.as_bytes();

    let start_index: usize = 0;
    for index in 0..as_bytes.len() {
        if as_bytes[index] == b'\n' {
            let line_vec = Vec::from(&as_bytes[start_index..index]);
            let line = String::from_utf8(line_vec).unwrap();

            all_lines.push(line);
        }
    }

    return all_lines;
}


pub fn tally(match_results: &str) -> String {
    let lines = parse_lines(match_results);

    for line in lines.iter() {
        print!("{}", &line);
    }

    unimplemented!(
        "Given the result of the played matches '{}' return a properly formatted tally table string.",
        match_results
    );
}
