use std::fs::read_to_string;

fn main() {
    let path = "src/input.txt";
    let input_file_contents = read_to_string(path).expect("something went wrong reading the file");

    let mut highest_cal_count = 0;
    let mut current_cal_count = 0;

    let mut all_scores: Vec<(i32, i32)> = vec![];
    let mut elf_counter = 0;

    input_file_contents.lines().for_each(|line| {
        if line.is_empty() {
            all_scores.push((elf_counter, current_cal_count));
            if current_cal_count > highest_cal_count {
                highest_cal_count = current_cal_count;
            }
            current_cal_count = 0;
            elf_counter += 1;
        } else {
            let line_as_int = line.parse::<i32>().expect("could not parse as i32");
            current_cal_count += line_as_int;
        }
    });

    all_scores.sort_by(|(_elf_a, score_a), (_elf_b, score_b)| score_b.cmp(score_a));

    let top_three_total_calories: i32 = (0..3)
        .zip(all_scores.iter())
        .map(|(_i, (_elf, score))| score)
        .sum();

    println!("highest_cal_count: {highest_cal_count}");
    println!("top_three_total_calories: {top_three_total_calories:?}");
}
