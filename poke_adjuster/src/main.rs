use stats_calculator::*;
fn main() {
    println!("Hello, world! {:?}", BasePoint(125));

    let ss = [1,2,3,4,5];

}

// Hp, Attack, Defense, SpAttack, SpDefense, Speed

trait User {

}

trait StatsLoader {
    fn calc_stats() -> Box<dyn User>;
}


#[derive(Debug)]
struct Sample {
    ss: u32,
    line: [i32; 5],
}
