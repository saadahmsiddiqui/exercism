#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list.len() == 0 && _second_list.len() != 0 {
        return Comparison::Sublist;
    }
    if _first_list.len() != 0 && _second_list.len() == 0 {
        return Comparison::Superlist;
    }
    if _first_list.len() == 0 && _second_list.len() == 0 {
        return Comparison::Equal;
    }

    let mut _second_list_iterator = 0;

    while _second_list_iterator < _second_list.len() {
        let mut _first_list_iterator = 0;
        let mut _matched_since_si = 0;
        let mut _shadow_second = _second_list_iterator;
        if _first_list.len() > _second_list.len() {
            while _first_list[_first_list_iterator].ne(&_second_list[_shadow_second]) && _first_list_iterator < _first_list.len() {
                _first_list_iterator = _first_list_iterator + 1;
            }
        }
        while _first_list_iterator < _first_list.len() {
            if _first_list[_first_list_iterator].eq(&_second_list[_shadow_second]) {
                _first_list_iterator = _first_list_iterator + 1;
                _shadow_second = _shadow_second + 1;
                _matched_since_si = _matched_since_si + 1;
            } else {
                _first_list_iterator = _first_list.len() + 1;
            }
            if _first_list_iterator == _first_list.len() {
                _first_list_iterator = _first_list.len() + 1;
            }
            if _shadow_second == _second_list.len() {
                _first_list_iterator = _first_list.len() + 1;
            }
        }


        if _matched_since_si == _first_list.len() || _matched_since_si == _second_list.len() {
            if _matched_since_si == _first_list.len() && _matched_since_si == _second_list.len() {
                return Comparison::Equal;
            }

            if _matched_since_si == _first_list.len() {
                return Comparison::Sublist;
            }


            return Comparison::Superlist;
        }


        _second_list_iterator = _second_list_iterator + 1;

    }

    return Comparison::Unequal;
}
