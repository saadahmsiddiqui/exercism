// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let production_per_hour: u8 = 221;
    let expected_production: f64 = (production_per_hour as f64) * (speed as f64);

    match speed {
        1..=4 => expected_production * 1.0,
        5..=8 => expected_production * 0.9,
        9..=10 => expected_production * 0.77,
        _ => expected_production * 0.0
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let production_per_hour = production_rate_per_hour(speed) as u32;
    let ppm: u32 = production_per_hour / 60;
    return ppm;
}
