use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

type PositionsPerAntenna = HashMap<char, HashSet<(usize, usize)>>;

fn parse_input(input: &String) -> (PositionsPerAntenna, usize, usize) {
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

fn is_node_in_range(antinode: (i32, i32), rows: usize, cols: usize) -> bool {
    antinode.0 >= 0
        && antinode.0 < rows.try_into().unwrap()
        && antinode.1 >= 0
        && antinode.1 < cols.try_into().unwrap()
}

pub fn main() {
    let input = fs::read_to_string("src/day08/input.txt").expect("could not read input file");
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    let (positions_per_antenna, rows, cols) = parse_input(&input);
    positions_per_antenna.values().for_each(|positions| {
        positions.iter().combinations(2).for_each(|combination| {
            let position_one = combination[0];
            let position_two = combination[1];

            let position_one_row = i32::try_from(position_one.0).unwrap();
            let position_one_col = i32::try_from(position_one.1).unwrap();
            let position_two_row = i32::try_from(position_two.0).unwrap();
            let position_two_col = i32::try_from(position_two.1).unwrap();

            let row_diff = position_two_row - position_one_row;
            let col_diff = position_two_col - position_one_col;

            let mut antinode = (position_one_row, position_one_col);
            while is_node_in_range(antinode, rows, cols) {
                antinodes.insert(antinode);
                antinode = (antinode.0 - row_diff, antinode.1 - col_diff);
            }
            antinode = (position_one_row, position_one_col);
            while is_node_in_range(antinode, rows, cols) {
                antinodes.insert(antinode);
                antinode = (antinode.0 + row_diff, antinode.1 + col_diff);
            }
        });
    });

    println!("{}", antinodes.len());
}
