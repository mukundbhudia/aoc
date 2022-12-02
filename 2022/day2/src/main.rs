use std::fs::read_to_string;

fn main() {
    let path = "src/input.txt";
    let input_file_contents = read_to_string(path).expect("something went wrong reading the file");

    // Part 1
    let score = input_file_contents
        .lines()
        .map(|x| x.split_whitespace().collect::<Vec<_>>())
        .fold(0, |acc, x| acc + calculate_score(x[0], x[1]));

    // Part 2
    let score_when_choosing_object = input_file_contents
        .lines()
        .map(|x| x.split_whitespace().collect::<Vec<_>>())
        .fold(0, |acc, x| acc + calculate_chose_to_win_score(x[0], x[1]));

    println!("score (part 1): {score}");
    println!("score (part 2): {score_when_choosing_object}");
}

fn calculate_score(opponent_object: &str, your_object: &str) -> i32 {
    let your_choice_score = match your_object {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => panic!("Input error. Your object should be X, Y or Z."),
    };

    let choice_score = match (opponent_object, your_object) {
        ("A", "X") => 3, // Rock, Rock  => draw
        ("B", "Y") => 3, // Paper, Paper  => draw
        ("C", "Z") => 3, // Scissors, Scissors  => draw
        ("A", "Y") => 6, // Rock, Paper  => win
        ("A", "Z") => 0, // Rock, Scissors  => loose
        ("B", "X") => 0, // Paper, Rock  => loose
        ("B", "Z") => 6, // Paper, Scissors  => win
        ("C", "X") => 6, // Scissors, Rock  => win
        ("C", "Y") => 0, // Scissors, Paper  => loose
        (_, _) => panic!("Input error. Bad object combination."),
    };

    your_choice_score + choice_score
}

fn calculate_chose_to_win_score(opponent_object: &str, your_object: &str) -> i32 {
    let choice_to_for_given_outcome = match (opponent_object, your_object) {
        ("A", "X") => "Z", // Given Rock, you need to loose => you choose scissors
        ("B", "Y") => "Y", // Given Paper, you need to draw => you choose paper
        ("C", "Z") => "X", // Given Scissors, you need to win => you choose rock
        ("A", "Y") => "X", // Given Rock, you need to draw => you choose rock
        ("A", "Z") => "Y", // Given Rock, you need to win => you choose paper
        ("B", "X") => "X", // Given Paper, you need to loose => you choose rock
        ("B", "Z") => "Z", // Given Paper, you need to win => you choose scissors
        ("C", "X") => "Y", // Given Scissors, you need to loose => you choose paper
        ("C", "Y") => "Z", // Given Scissors, you need to draw => you choose scissors
        (_, _) => panic!("Input error. Bad object combination."),
    };
    calculate_score(opponent_object, choice_to_for_given_outcome)
}
