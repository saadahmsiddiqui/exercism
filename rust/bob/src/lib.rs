const SURE: &str = "Sure.";
const YELL: &str = "Whoa, chill out!";
const CALM: &str = "Calm down, I know what I'm doing!";
const FINE: &str = "Fine. Be that way!";
const WHATEVER: &str = "Whatever.";

pub fn reply(message: &str) -> &str {
    if message.len() == 0 {
        return FINE;
    }

    let is_question = message.trim_ascii_end().ends_with("?");
    let mut uppercase = 0;
    let mut lowercase = 0;
    let mut digits = 0;
    let mut spaces = 0;
    let mut special_chars = 0;

    message.chars().into_iter().for_each(|f| {
        let ascii = f as u32;
        if ascii >= 65 && ascii <= 90 {
            uppercase += 1;
        } else if ascii >= 97 && ascii <= 122 {
            lowercase += 1;
        } else if ascii >= 48 && ascii <= 57 {
            digits += 1;
        } else if ascii == 32 || f == '\t' {
            spaces += 1;
        } else if ascii >= 33 && ascii <= 47 {
            special_chars += 1;
        }
    });


    if is_question {
        if uppercase > 0 {
            if lowercase == 0 {
                return CALM;
            }
        }

        return SURE;
    }

    if lowercase == 0 && uppercase > 0 && !is_question {
        return YELL;
    }

    if spaces > 0 && uppercase == 0 && lowercase == 0 && special_chars == 0 {
        return FINE;
    }

    return WHATEVER;
}
