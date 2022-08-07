use std::collections::HashMap;

pub fn can_construct_note(magazine: &Vec<&str>, note: &Vec<&str>) -> bool {
    let mut megazine_map: HashMap<String, u8> = HashMap::new();

    for word in magazine {
        let w = String::from(*word);

        if megazine_map.contains_key(&w) {
            megazine_map.entry(w).and_modify(|e| *e += 1);
        } else {
            megazine_map.insert(w, 1);
        }
    }

    let mut note_words = note.len();

    for word in note.iter() {
        let note_w = String::from(*word);

        let mut remove_key = false;
        let mut decrease_count = false;
        match megazine_map.get_key_value(&note_w) {
            Some(key_match) => {
                let (_word, count) = key_match;

                if *count == 0 {
                    remove_key = true;
                } else {
                    note_words -= 1;
                    decrease_count = true;
                }
            }
            None => return false,
        }

        if remove_key {
            megazine_map.remove_entry(&note_w);
        }

        if decrease_count {
            megazine_map.entry(note_w).and_modify(|e| *e -= 1);
        }
    }

    if note_words == 0 {
        return true;
    }

    return false;
}
