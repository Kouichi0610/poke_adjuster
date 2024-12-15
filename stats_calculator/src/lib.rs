#[derive(PartialEq, Eq, Debug)]
pub struct Level(pub u32);

#[derive(PartialEq, Eq, Debug)]
pub struct Species(pub u32);

#[derive(PartialEq, Eq, Debug)]
pub struct Individual(pub u32);

#[derive(PartialEq, Eq, Debug)]
pub struct BasePoint(pub u32);

#[derive(PartialEq, Eq, Debug)]
pub struct Stats(pub u32);

pub trait StatsCorrector {
    fn calculate() -> u32;
}

pub fn calculate_hp(l: Level, s: Species, i: Individual, b: BasePoint) -> Stats {
    let a = (s.0 * 2 + i.0 + b.0 / 4) as f32;
    let lv = l.0 as f32;
    let y = a * lv / 100.0 + lv + 10.0;
    Stats(y as u32)
}

pub fn calculate_shadinja_hp(_l: Level, _s: Species, _i: Individual, _b: BasePoint) -> Stats {
    Stats(1)
}

pub fn calculate_flat(l: Level, s: Species, i: Individual, b: BasePoint) -> Stats {
    let a = (s.0 * 2 + i.0 + b.0 / 4) as f32;
    let lv = l.0 as f32;
    let b = a * lv / 100.0 + 5.0;

    Stats(b as u32)
}
pub fn calculate_upper(l: Level, s: Species, i: Individual, b: BasePoint) -> Stats {
    let stats = calculate_flat(l, s, i, b);
    Stats(stats.0 * 11 / 10)
}
pub fn calculate_lower(l: Level, s: Species, i: Individual, b: BasePoint) -> Stats {
    let stats = calculate_flat(l, s, i, b);
    Stats(stats.0 * 9 / 10)
}

