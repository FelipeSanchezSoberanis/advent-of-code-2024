use std::fs;

pub fn main() {
    let input = fs::read_to_string("src/day04/input.txt").expect("could not read input file");

    let mut array: Vec<Vec<char>> = vec![];
    let mut row = 0;
    input.lines().for_each(|line| {
        array.push(Vec::new());
        line.chars().for_each(|char| array[row].push(char));
        row += 1;
    });

    let rows = array.len();
    let cols = array[0].len();

    let mut res = 0;

    for row in 1..rows - 1 {
        for col in 1..cols - 1 {
            if array[row][col] != 'A' {
                continue;
            }

            let top_left = array[row - 1][col - 1];
            let top_right = array[row - 1][col + 1];
            let bottom_left = array[row + 1][col - 1];
            let bottom_right = array[row + 1][col + 1];
            let chars = vec![top_left, top_right, bottom_right, bottom_left];

            if !surrounded_by(&chars, 2, 'M')
                || !surrounded_by(&chars, 2, 'S')
                || top_left == bottom_right
            {
                continue;
            }

            res += 1;
        }
    }

    println!("{res}");
}

fn surrounded_by(chars: &Vec<char>, count: usize, char: char) -> bool {
    chars
        .iter()
        .filter(|&&c| c == char)
        .collect::<Vec<_>>()
        .len()
        == count
}
