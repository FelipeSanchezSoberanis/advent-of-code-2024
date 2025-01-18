use std::fs;

fn parse_input(input: String) -> Vec<String> {
    input.lines().collect::<Vec<_>>()[0]
        .chars()
        .enumerate()
        .map(|(i, char)| {
            let number = char.to_digit(10).expect("could not parse char to u8");
            let mut slice: Vec<String> = Vec::new();
            for _ in 0..number {
                if i % 2 == 0 {
                    slice.push((i / 2).to_string());
                } else {
                    slice.push(String::from("."));
                }
            }
            slice
        })
        .fold(Vec::new(), |mut acc, mut slice| {
            acc.append(&mut slice);
            acc
        })
}

fn reorder_filesystem(filesystem: &mut Vec<String>) {
    let mut left = 0;
    let mut right = filesystem.len() - 1;
    while left < right {
        while filesystem[left] != "." {
            left += 1;
        }
        while filesystem[right] == "." {
            right -= 1;
        }
        filesystem.swap(left, right);
        left += 1;
        right -= 1;
    }
}

fn calculate_checksum(filesystem: &Vec<String>) -> u64 {
    filesystem
        .iter()
        .enumerate()
        .fold(0, |mut acc, (index, slice)| {
            if slice == "." {
                return acc;
            }
            acc += u64::try_from(index).expect("could not parse usize into u64")
                * slice.parse::<u64>().expect("could not parse string to u64");
            acc
        })
}

pub fn main() {
    let input = fs::read_to_string("src/day09/input.txt").expect("could not read input file");
    let mut filesystem = parse_input(input);
    reorder_filesystem(&mut filesystem);
    let checksum = calculate_checksum(&filesystem);
    println!("{checksum}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_checksum_one() {
        let input = "0099811188827773336446555566.............."
            .chars()
            .map(|char| String::from(char))
            .collect::<Vec<String>>();
        let expected = 1928;
        let actual = calculate_checksum(&input);
        assert_eq!(expected, actual, "input: {input:?}");
    }

    #[test]
    fn test_calculate_checksum_two() {
        let input = "00992111777.44.333....5555.6666.....8888.."
            .chars()
            .map(|char| String::from(char))
            .collect::<Vec<String>>();
        let expected = 2858;
        let actual = calculate_checksum(&input);
        assert_eq!(expected, actual, "input: {input:?}");
    }
}
