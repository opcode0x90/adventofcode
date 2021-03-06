use std::fs::File;

use aoc::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // read data from input.txt
    let f = File::open("input.txt").expect("input.txt not found!");
    let input = get_input(f)?;

    let part1 = part1(&input);
    println!("part1: {}", part1);

    let part2 = part2(&input, 5, 60);
    println!("part2: {}", part2);

    Ok(())
}
