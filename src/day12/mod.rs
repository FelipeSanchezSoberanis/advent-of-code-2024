use std::{
    collections::{HashSet, VecDeque},
    fs,
};

fn parse_input(filepath: &str) -> Vec<Vec<char>> {
    fs::read_to_string(filepath)
        .expect("could not read input")
        .lines()
        .fold(Vec::new(), |mut acc, line| {
            acc.push(line.chars().collect());
            acc
        })
}

fn part_one(filepath: &str) -> u128 {
    let map = parse_input(filepath);

    let deltas = Vec::<(isize, isize)>::from([(-1, 0), (0, -1), (1, 0), (0, 1)]);
    let mut visited = HashSet::new();

    map.iter()
        .enumerate()
        .fold(Vec::new(), |mut acc, (row, line)| {
            line.iter()
                .enumerate()
                .for_each(|(col, _)| acc.push((row, col)));
            acc
        })
        .iter()
        .fold(Vec::new(), |mut tracker, &(start_row, start_col)| {
            if visited.contains(&(start_row, start_col)) {
                return tracker;
            }

            let mut area = 0;
            let mut perimeter = 0;
            let char = map[start_row][start_col];

            let mut queue = VecDeque::new();

            queue.push_back((start_row, start_col));
            visited.insert((start_row, start_col));
            while let Some((center_row, center_col)) = queue.pop_front() {
                area += 1;

                deltas
                    .iter()
                    .map(|(delta_row, delta_col)| {
                        (
                            center_row as isize + delta_row,
                            center_col as isize + delta_col,
                        )
                    })
                    .for_each(|(surr_row, surr_col)| {
                        let is_cell_inbounds = surr_row >= 0
                            && surr_row <= map.len() as isize - 1
                            && surr_col >= 0
                            && surr_col <= map[0].len() as isize - 1;
                        let is_same_char = match is_cell_inbounds {
                            true => map[surr_row as usize][surr_col as usize] == char,
                            false => false,
                        };

                        if is_cell_inbounds && is_same_char {
                            if !visited.contains(&(surr_row as usize, surr_col as usize)) {
                                queue.push_back((surr_row as usize, surr_col as usize));
                                visited.insert((surr_row as usize, surr_col as usize));
                            }
                        } else {
                            perimeter += 1;
                        }
                    });
            }

            tracker.push((area, perimeter));
            tracker
        })
        .iter()
        .map(|(area, perimeter)| area * perimeter)
        .sum()
}

fn part_two(filepath: &str) -> u128 {
    let map = parse_input(filepath);

    let deltas = Vec::<(isize, isize)>::from([
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
    ]);
    let mut visited = HashSet::new();

    map.iter()
        .enumerate()
        .fold(Vec::new(), |mut acc, (row, line)| {
            line.iter()
                .enumerate()
                .for_each(|(col, _)| acc.push((row, col)));
            acc
        })
        .iter()
        .fold(Vec::new(), |mut tracker, &(start_row, start_col)| {
            if visited.contains(&(start_row, start_col)) {
                return tracker;
            }

            let mut area = 0;
            let mut corners = 0;
            let char = map[start_row][start_col];

            let mut queue = VecDeque::new();

            queue.push_back((start_row, start_col));
            visited.insert((start_row, start_col));
            while let Some((center_row, center_col)) = queue.pop_front() {
                area += 1;

                let mut corner_finder = Vec::new();

                deltas
                    .iter()
                    .map(|(delta_row, delta_col)| {
                        (
                            center_row as isize + delta_row,
                            center_col as isize + delta_col,
                            delta_row.abs() == delta_col.abs(),
                        )
                    })
                    .for_each(|(surr_row, surr_col, is_diagonal)| {
                        let is_cell_inbounds = surr_row >= 0
                            && surr_row <= map.len() as isize - 1
                            && surr_col >= 0
                            && surr_col <= map[0].len() as isize - 1;
                        let is_same_char = match is_cell_inbounds {
                            true => map[surr_row as usize][surr_col as usize] == char,
                            false => false,
                        };

                        if is_cell_inbounds && is_same_char {
                            if !is_diagonal
                                && !visited.contains(&(surr_row as usize, surr_col as usize))
                            {
                                queue.push_back((surr_row as usize, surr_col as usize));
                                visited.insert((surr_row as usize, surr_col as usize));
                            }

                            corner_finder.push(true);
                        } else {
                            corner_finder.push(false);
                        }

                        if corner_finder.len() >= 3
                            && !is_diagonal
                            && (!corner_finder[corner_finder.len() - 3]
                                && !corner_finder[corner_finder.len() - 1]
                                || corner_finder[corner_finder.len() - 3]
                                    && !corner_finder[corner_finder.len() - 2]
                                    && corner_finder[corner_finder.len() - 1])
                        {
                            corners += 1;
                        }
                    });
            }

            println!("{char} {area} {corners}");
            tracker.push((area, corners));
            tracker
        })
        .iter()
        .map(|(area, corners)| area * corners)
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day12::{part_one, part_two};

    #[test]
    fn test_part_one_example_one() {
        let expected = 140;
        let actual = part_one("./src/day12/input.example.one.txt");
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_one_example_two() {
        let expected = 772;
        let actual = part_one("./src/day12/input.example.two.txt");
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_one_example_three() {
        let expected = 1930;
        let actual = part_one("./src/day12/input.example.three.txt");
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_one() {
        let expected = 1473620;
        let actual = part_one("./src/day12/input.txt");
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_two_example_one() {
        let expected = 80;
        let actual = part_two("./src/day12/input.example.one.txt");
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_two_example_two() {
        let expected = 436;
        let actual = part_two("./src/day12/input.example.two.txt");
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_two_example_four() {
        let expected = 236;
        let actual = part_two("./src/day12/input.example.four.txt");
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_two_example_five() {
        let expected = 368;
        let actual = part_two("./src/day12/input.example.five.txt");
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_two() {
        let expected = 902620;
        let actual = part_two("./src/day12/input.txt");
        assert_eq!(expected, actual);
    }
}
