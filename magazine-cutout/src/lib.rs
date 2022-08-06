// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &Vec<&str>, note: &Vec<&str>) -> bool {
    let mut megazine_map: HashMap<String, u8> = HashMap::new();

    for word in magazine {
        let w = String::from(*word);

        if (megazine_map.contains_key(&w)) {
            megazine_map.entry(w).and_modify(|e| *e += 1);
        } else {
            megazine_map.insert(w, 1);
        }
    }
    unimplemented!()
}
