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

    let mut horizontal_string = String::from("");
    for row in 0..rows {
        for col in 0..cols {
            horizontal_string.push(array[row][col]);
        }
        horizontal_string.push(' ');
    }

    let mut vertical_string = String::from("");
    for col in 0..cols {
        for row in 0..rows {
            vertical_string.push(array[row][col]);
        }
        vertical_string.push(' ');
    }

    let mut diagonal_down_string = String::from("");
    for row in 0..rows - 3 {
        for col in 0..cols - 3 {
            for i in 0..4 {
                diagonal_down_string.push(array[row + i][col + i]);
            }
            diagonal_down_string.push(' ');
        }
    }

    let mut diagonal_up_string = String::from("");
    for row in 3..rows {
        for col in 0..cols - 3 {
            for i in 0..4 {
                diagonal_up_string.push(array[row - i][col + i]);
            }
            diagonal_up_string.push(' ');
        }
    }

    let final_string = String::from("")
        + &horizontal_string
        + &vertical_string
        + &diagonal_down_string
        + &diagonal_up_string;

    let res = final_string.matches("XMAS").collect::<Vec<&str>>().len()
        + final_string
            .chars()
            .rev()
            .collect::<String>()
            .matches("XMAS")
            .collect::<Vec<&str>>()
            .len();

    println!("{res}");
}
