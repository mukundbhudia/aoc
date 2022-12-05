use std::fs::read_to_string;

fn main() {
    let path = "src/input.txt";
    let input_file_contents = read_to_string(path).expect("something went wrong reading the file");

    let part_1 = part_1(input_file_contents.clone());
    println!("part 1 answer: {part_1}");

    let part_2 = part_2(input_file_contents);
    println!("part 2 answer: {part_2}");
}

fn part_1(input: String) -> i32 {
    input
        .lines()
        .fold(0, |acc, l| acc + get_common_item_in_compartments(l))
}

fn part_2(input: String) -> i32 {
    input
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .fold(0, |acc, l| {
            acc + get_common_item_in_three_rucksacks(l[0], l[1], l[2])
        })
}

fn get_common_item_in_compartments(all_items: &str) -> i32 {
    let (first_compartment, second_compartment) = all_items.split_at(all_items.len() / 2);

    let common_item_in_compartment = first_compartment
        .chars()
        .find(|item| second_compartment.contains(&item.to_string()))
        .unwrap();

    get_priorities_for_item(common_item_in_compartment)
}

fn get_common_item_in_three_rucksacks(rucksack_1: &str, rucksack_2: &str, rucksack_3: &str) -> i32 {
    let common_item_in_compartment = rucksack_1
        .chars()
        .find(|item| {
            rucksack_2.contains(&item.to_string()) && rucksack_3.contains(&item.to_string())
        })
        .unwrap();

    get_priorities_for_item(common_item_in_compartment)
}

fn get_priorities_for_item(item: char) -> i32 {
    match item {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        'A' => 27,
        'B' => 28,
        'C' => 29,
        'D' => 30,
        'E' => 31,
        'F' => 32,
        'G' => 33,
        'H' => 34,
        'I' => 35,
        'J' => 36,
        'K' => 37,
        'L' => 38,
        'M' => 39,
        'N' => 40,
        'O' => 41,
        'P' => 42,
        'Q' => 43,
        'R' => 44,
        'S' => 45,
        'T' => 46,
        'U' => 47,
        'V' => 48,
        'W' => 49,
        'X' => 50,
        'Y' => 51,
        'Z' => 52,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_given_test() {
        let test_input_file = r##"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"##
            .to_string();

        assert_eq!(part_1(test_input_file), 157);
    }

    #[test]
    fn part2_given_test() {
        let test_input_file = r##"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"##
            .to_string();

        assert_eq!(part_2(test_input_file), 70);
    }
}
