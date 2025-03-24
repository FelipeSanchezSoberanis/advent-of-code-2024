use std::{collections::HashSet, fs};

fn parse_input(filepath: &str) -> Vec<Vec<u32>> {
    let mut map: Vec<Vec<u32>> = Vec::new();
    fs::read_to_string(filepath)
        .expect("could not parse input")
        .lines()
        .for_each(|row| {
            map.push(
                row.chars()
                    .map(|char| char.to_digit(10).expect("could not parse number"))
                    .collect::<Vec<_>>(),
            );
        });
    map
}

fn get_surrounding_cells(
    map: &Vec<Vec<u32>>,
    center_cell: &(usize, usize),
    counted_cells: &HashSet<(usize, usize)>,
) -> Vec<(usize, usize)> {
    let mut surrounding_cells = Vec::new();

    let rows = map.len() as i8;
    let cols = map[0].len() as i8;

    for i in 0..3 {
        for j in 0..3 {
            let i_delta = i - 1 as i8;
            let j_delta = j - 1 as i8;
            if i_delta == 0 && j_delta == 0
                || i_delta != 0 && j_delta != 0
                || center_cell.0 as i8 + i_delta < 0
                || center_cell.0 as i8 + i_delta > rows - 1
                || center_cell.1 as i8 + j_delta < 0
                || center_cell.1 as i8 + j_delta > cols - 1
                || map[(center_cell.0 as i8 + i_delta) as usize]
                    [(center_cell.1 as i8 + j_delta) as usize] as i32
                    - map[center_cell.0 as usize][center_cell.1 as usize] as i32
                    != 1
                || counted_cells.contains(&(
                    (center_cell.0 as i8 + i_delta) as usize,
                    (center_cell.1 as i8 + j_delta) as usize,
                ))
            {
                continue;
            }

            surrounding_cells.push((
                (center_cell.0 as i8 + i_delta) as usize,
                (center_cell.1 as i8 + j_delta) as usize,
            ));
        }
    }

    surrounding_cells
}

fn part_one(filepath: &str) -> u32 {
    let map = parse_input(filepath);

    let mut stack = Vec::<(usize, usize)>::new();
    let mut score = 0;

    map.iter().enumerate().for_each(|(row_index, row)| {
        row.iter().enumerate().for_each(|(col_index, cell)| {
            if *cell != 0 {
                return;
            }

            let mut counted_cells = HashSet::<(usize, usize)>::new();

            stack.push((row_index, col_index));
            while let Some(last_cell) = stack.pop() {
                if map[last_cell.0][last_cell.1] == 9 {
                    score += 1;
                    counted_cells.insert(last_cell);
                } else {
                    get_surrounding_cells(&map, &last_cell, &counted_cells)
                        .iter()
                        .for_each(|surrounding_cell| stack.push(*surrounding_cell));
                }
            }
        });
    });

    score
}

fn part_two(filepath: &str) -> u32 {
    let map = parse_input(filepath);

    let mut stack = Vec::<(usize, usize)>::new();
    let mut score = 0;
    let counted_cells = HashSet::<(usize, usize)>::new();

    map.iter().enumerate().for_each(|(row_index, row)| {
        row.iter().enumerate().for_each(|(col_index, cell)| {
            if *cell != 0 {
                return;
            }

            stack.push((row_index, col_index));
            while let Some(last_cell) = stack.pop() {
                if map[last_cell.0][last_cell.1] == 9 {
                    score += 1;
                } else {
                    get_surrounding_cells(&map, &last_cell, &counted_cells)
                        .iter()
                        .for_each(|surrounding_cell| stack.push(*surrounding_cell));
                }
            }
        });
    });

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example() {
        let expected = 36;
        let actual = part_one("./src/day10/input.example.txt");
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_one() {
        let expected = 535;
        let actual = part_one("./src/day10/input.txt");
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_two_example() {
        let expected = 81;
        let actual = part_two("./src/day10/input.example.txt");
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_two() {
        let expected = 1186;
        let actual = part_two("./src/day10/input.txt");
        assert_eq!(expected, actual);
    }
}
