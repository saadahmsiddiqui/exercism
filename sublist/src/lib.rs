use std::cmp;

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    let mut _second_iterator = 0;
    let mut _match_count = 0;

    while _second_iterator < _second_list.len() {
        
        let mut _first_iterator = 0;
        let mut _sec = _second_iterator;
        let mut mc = 0;
        while _sec < _second_list.len() && _first_iterator < _first_list.len() {
            if _first_list[_first_iterator].eq(&_second_list[_sec]) {
                mc = mc + 1;
                _first_iterator = _first_iterator + 1;
                _sec = _sec + 1;
            } else {
                _first_iterator = _first_list.len() + 1;
                _sec = _second_list.len() + 1
            }
        }

        if mc == _first_list.len() || mc == _second_list.len() {
            _match_count = cmp::max(_match_count, mc);
            break;
        }

        _match_count = cmp::max(_match_count, mc);
        _second_iterator = _second_iterator + 1;
    }

    if _match_count == 0 {
        if _first_list.len() == 0 && _second_list.len() != 0 {
            return Comparison::Sublist;
        }
        if _first_list.len() != 0 && _second_list.len() == 0 {
            return Comparison::Superlist;
        }
    }

    if _match_count == _first_list.len() && _match_count == _second_list.len() {
        return Comparison::Equal;
    }

    if _match_count == _first_list.len() {
        return Comparison::Sublist;
    }

    if _match_count == _second_list.len() {
        return Comparison::Superlist;
    }

    return Comparison::Unequal;

}
