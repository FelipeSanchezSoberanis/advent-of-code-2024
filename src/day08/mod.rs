use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

type PositionsPerAntenna = HashMap<char, HashSet<(usize, usize)>>;

fn parse_input(input: String) -> (PositionsPerAntenna, usize, usize) {
    let mut positions_per_antenna: PositionsPerAntenna = HashMap::new();
    input.lines().enumerate().for_each(|(row, line)| {
        line.chars().enumerate().for_each(|(col, char)| {
            if char == '.' {
                return;
            }
            if !positions_per_antenna.contains_key(&char) {
                positions_per_antenna.insert(char, HashSet::new());
            }
            if let Some(positions) = positions_per_antenna.get_mut(&char) {
                positions.insert((row, col));
            }
        });
    });
    (
        positions_per_antenna,
        input.lines().collect::<Vec<_>>().len(),
        input.lines().collect::<Vec<_>>()[0].len(),
    )
}

fn is_node_in_range(antinode_one: (i32, i32), rows: usize, cols: usize) -> bool {
    antinode_one.0 >= 0
        && antinode_one.0 < rows.try_into().unwrap()
        && antinode_one.1 >= 0
        && antinode_one.1 < cols.try_into().unwrap()
}

pub fn main() {
    let input = fs::read_to_string("src/day08/input.txt").expect("could not read input file");
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    let (positions_per_antenna, rows, cols) = parse_input(input);
    positions_per_antenna.values().for_each(|positions| {
        positions.iter().combinations(2).for_each(|combination| {
            let position_one = combination[0];
            let position_two = combination[1];
            let row_diff =
                i32::try_from(position_two.0).unwrap() - i32::try_from(position_one.0).unwrap();
            let col_diff =
                i32::try_from(position_two.1).unwrap() - i32::try_from(position_one.1).unwrap();
            let antinode_one = (
                i32::try_from(position_one.0).unwrap() - row_diff,
                i32::try_from(position_one.1).unwrap() - col_diff,
            );
            let antinode_two = (
                i32::try_from(position_two.0).unwrap() + row_diff,
                i32::try_from(position_two.1).unwrap() + col_diff,
            );
            if is_node_in_range(antinode_one, rows, cols) {
                antinodes.insert(antinode_one);
            }
            if is_node_in_range(antinode_two, rows, cols) {
                antinodes.insert(antinode_two);
            }
        });
    });
    println!("{}", antinodes.len());
}
