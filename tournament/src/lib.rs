// enum MatchState {
//     WIN,
//     LOSE,
//     DRAW
// }

// struct TallyStats {
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
        if as_bytes[i] == b';' || as_bytes[i] == b'\n' || i + 1 == as_bytes.len() {
            let value = Vec::from(&raw_str[index..i]);
            let utf8_str = String::from_utf8(value).unwrap();
            values_vec.push(utf8_str);
            index = i + 1;
        }
    }

    return values_vec;
}


pub fn tally(match_results: &str) -> String {
    let lines = parse_lines(match_results);
    // let teams: Vec<TallyStats> = Vec::new();

    for line in lines.iter() {
        let vals = parse_semi_color_separated_values(line.clone());
        println!("Parsed: {} {} {}\n", &vals[0], &vals[1], &vals[2]);
    }

    unimplemented!(
        "Given the result of the played matches '{}' return a properly formatted tally table string.",
        match_results
    );
}
