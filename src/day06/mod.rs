use std::{collections::HashSet, fs};

enum CellState {
    CLEAR,
    OBSTRUCTED,
    OUTSIDE,
}

#[derive(Debug, Default)]
enum Direction {
    #[default]
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

impl Direction {
    fn from_char(char: char) -> Direction {
        if char == '^' {
            return Direction::UP;
        }
        if char == '>' {
            return Direction::RIGHT;
        }
        if char == 'v' {
            return Direction::DOWN;
        }
        if char == '<' {
            return Direction::LEFT;
        }
        panic!("");
    }
}

#[derive(Debug, Default)]
struct Guard {
    row: usize,
    col: usize,
    dir: Direction,
}

impl Guard {
    fn change_direction(&mut self) {
        match self.dir {
            Direction::UP => self.dir = Direction::RIGHT,
            Direction::RIGHT => self.dir = Direction::DOWN,
            Direction::DOWN => self.dir = Direction::LEFT,
            Direction::LEFT => self.dir = Direction::UP,
        }
    }

    fn take_step(&mut self) {
        match self.dir {
            Direction::UP => self.row -= 1,
            Direction::RIGHT => self.col += 1,
            Direction::DOWN => self.row += 1,
            Direction::LEFT => self.col -= 1,
        }
    }
}

#[derive(Debug)]
struct Map {
    cells: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
    guard: Guard,
}

impl Map {
    fn from_string(input: String) -> Map {
        let mut cells: Vec<Vec<char>> = Vec::new();
        let mut guard = Guard::default();

        for (line_index, line) in input.lines().enumerate() {
            let mut row = Vec::new();
            for (char_index, char) in line.chars().enumerate() {
                if vec!['^', '>', 'v', '<'].contains(&char) {
                    guard = Guard {
                        row: line_index,
                        col: char_index,
                        dir: Direction::from_char(char),
                    };
                    row.push('.');
                } else {
                    row.push(char);
                }
            }
            cells.push(row);
        }

        let rows = cells.len();
        let cols = cells[0].len();

        Map {
            cells,
            guard,
            rows,
            cols,
        }
    }

    fn get_next_step_cell_state(&self) -> CellState {
        let obstacles = vec!['#'];
        let row = self.guard.row;
        let col = self.guard.col;
        match self.guard.dir {
            Direction::UP => {
                if row == 0 {
                    return CellState::OUTSIDE;
                }
                if obstacles.contains(&self.cells[row - 1][col]) {
                    return CellState::OBSTRUCTED;
                }
                return CellState::CLEAR;
            }
            Direction::RIGHT => {
                if col == self.cols - 1 {
                    return CellState::OUTSIDE;
                }
                if obstacles.contains(&self.cells[row][col + 1]) {
                    return CellState::OBSTRUCTED;
                }
                return CellState::CLEAR;
            }
            Direction::DOWN => {
                if row == self.rows - 1 {
                    return CellState::OUTSIDE;
                }
                if obstacles.contains(&self.cells[row + 1][col]) {
                    return CellState::OBSTRUCTED;
                }
                return CellState::CLEAR;
            }
            Direction::LEFT => {
                if col == 0 {
                    return CellState::OUTSIDE;
                }
                if obstacles.contains(&self.cells[row][col - 1]) {
                    return CellState::OBSTRUCTED;
                }
                return CellState::CLEAR;
            }
        }
    }
}

pub fn main() {
    let input = fs::read_to_string("src/day06/input.txt").expect("could not read input file");
    let mut positions: HashSet<(usize, usize)> = HashSet::new();
    let mut map = Map::from_string(input);
    while !matches!(map.get_next_step_cell_state(), CellState::OUTSIDE) {
        positions.insert((map.guard.row, map.guard.col));
        while matches!(map.get_next_step_cell_state(), CellState::OBSTRUCTED) {
            map.guard.change_direction();
        }
        map.guard.take_step();
    }
    positions.insert((map.guard.row, map.guard.col));
    println!("{}", positions.len());
}
