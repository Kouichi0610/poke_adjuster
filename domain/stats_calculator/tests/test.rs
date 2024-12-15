
use stats_calculator::*;

#[test]
fn test_calculate_hp() {
    let l = Level(50);
    let s = Species(80);
    let i = Individual(31);
    let b = BasePoint(252);

    let actual = calculate_hp(l, s, i, b);
    assert_eq!(actual, Stats(187));
}
#[test]
fn test_calculate_shadinja_hp()
{
    let l = Level(50);
    let s = Species(80);
    let i = Individual(31);
    let b = BasePoint(252);

    let actual = calculate_shadinja_hp(l, s, i, b);
    assert_eq!(actual, Stats(1));
}
#[test]
fn test_calculate_flat()
{
    let l = Level(50);
    let s = Species(80);
    let i = Individual(31);
    let b = BasePoint(252);

    let actual = calculate_flat(l, s, i, b);
    assert_eq!(actual, Stats(132));
}
#[test]
fn test_calculate_upper()
{
    let l = Level(50);
    let s = Species(80);
    let i = Individual(31);
    let b = BasePoint(252);

    let actual = calculate_upper(l, s, i, b);
    assert_eq!(actual, Stats(145));
}
#[test]
fn test_calculate_lower()
{
    let l = Level(50);
    let s = Species(80);
    let i = Individual(31);
    let b = BasePoint(252);

    let actual = calculate_lower(l, s, i, b);
    assert_eq!(actual, Stats(118));
}
