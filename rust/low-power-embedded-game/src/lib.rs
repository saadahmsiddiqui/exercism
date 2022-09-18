// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn divmod(dividend: i16, divisor: i16) -> (i16, i16) {
    let division = dividend / divisor;
    let modulus = dividend % divisor;
    return (division, modulus);
}

pub fn evens<T>(iter: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
    return iter.enumerate().filter(|(i, el)| i % 2 == 0).map(|(i, el)| el);
}

pub struct Position(pub i16, pub i16);
impl Position {
    pub fn manhattan(&self) -> i16 {
        let distanceX = (0 - self.0).abs();
        let distanceY = (0 - self.1).abs();

        distanceX + distanceY
    }
}
