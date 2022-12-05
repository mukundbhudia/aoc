use std::{fs::read_to_string, ops::Range};

fn main() {
    let path = "src/input.txt";
    let input_file_contents = read_to_string(path).expect("something went wrong reading the file");

    let part_1 = part_1(input_file_contents.clone());
    println!("part 1 answer: {part_1}");

    let part_2 = part_2(input_file_contents);
    println!("part 2 answer: {part_2}");
}

fn part_1(input: String) -> i32 {
    input.lines().map(get_subset_range).sum()
}

fn part_2(input: String) -> i32 {
    input.lines().map(get_overlaps).sum()
}

fn get_ranges(pairs: &str) -> (Range<i32>, Range<i32>) {
    let pairs = pairs.split(',').collect::<Vec<_>>();

    let first_pair_start = pairs[0].split('-').next().unwrap().parse::<i32>().unwrap();
    let first_pair_end = pairs[0].split('-').nth(1).unwrap().parse::<i32>().unwrap();

    let second_pair_start = pairs[1].split('-').next().unwrap().parse::<i32>().unwrap();
    let second_pair_end = pairs[1].split('-').nth(1).unwrap().parse::<i32>().unwrap();

    (
        first_pair_start..(first_pair_end + 1),
        second_pair_start..(second_pair_end + 1),
    )
}

fn get_subset_range(line: &str) -> i32 {
    let (first_pair, second_pair) = get_ranges(line);

    let all_of_first_in_second = first_pair.clone().all(|x| second_pair.contains(&x));
    let all_of_second_in_first = second_pair.into_iter().all(|x| first_pair.contains(&x));

    i32::from(all_of_second_in_first || all_of_first_in_second)
}

fn get_overlaps(line: &str) -> i32 {
    let (first_pair, second_pair) = get_ranges(line);

    let any_of_first_in_second = first_pair.into_iter().any(|x| second_pair.contains(&x));

    i32::from(any_of_first_in_second)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_given_test() {
        let test_input_file = r##"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"##
            .to_string();

        assert_eq!(part_1(test_input_file), 2);
    }

    #[test]
    fn part2_given_test() {
        let test_input_file = r##"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"##
            .to_string();

        assert_eq!(part_2(test_input_file), 4);
    }
}
