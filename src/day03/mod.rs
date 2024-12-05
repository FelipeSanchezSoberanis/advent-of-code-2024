use std::fs;

pub fn main() {
    let mut total = 0;

    let input = fs::read_to_string("src/day03/input.txt").expect("could not read input file");

    let dos = input
        .match_indices("do()")
        .map(|(i, _)| i)
        .collect::<Vec<_>>();
    let donts = input
        .match_indices("don't()")
        .map(|(i, _)| i)
        .collect::<Vec<_>>();

    input.match_indices("mul(").for_each(|(start_index, _)| {
        let remaining = &input[start_index..];
        let instruction = match remaining.find(")") {
            Some(end_index) => &remaining[..end_index + 1],
            None => return,
        };
        if instruction.match_indices(",").collect::<Vec<_>>().len() != 1 {
            return;
        }
        let parenthesis_content = &instruction[4..instruction.len() - 1];
        let instruction_parts = parenthesis_content.split(",");
        let mut instruction_part_is_valid =
            instruction_parts.map(|part| part.chars().all(char::is_numeric));
        if !instruction_part_is_valid.all(|f| f == true) {
            return;
        };

        let operation_enabled = match donts.iter().filter(|&&i| i < start_index).max() {
            Some(prev_dont) => match dos.iter().filter(|&&i| i < start_index).max() {
                Some(prev_do) => prev_do > prev_dont,
                None => false,
            },
            None => true,
        };

        if !operation_enabled {
            return;
        };

        let numbers = parenthesis_content
            .split(",")
            .map(|number_string| {
                number_string
                    .parse::<u32>()
                    .expect("could not parse number string")
            })
            .collect::<Vec<_>>();
        total += numbers[0] * numbers[1];
    });

    println!("{}", total);
}
