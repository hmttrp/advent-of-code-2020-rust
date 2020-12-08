use std::fs;

const INPUT_FILE: &str = "./input/day05.txt";
// const INPUT_FILE: &str = "./input/day05_example.txt";

fn generate_seat_id(seat_string: &str) -> i32 {
    let (row_string, column_string) = seat_string.split_at(7);

    let row_binary = row_string.replace("F", "0").replace("B", "1");
    let column_binary = column_string.replace("L", "0").replace("R", "1");

    let row = i32::from_str_radix(&row_binary, 2).unwrap();
    let column = i32::from_str_radix(&column_binary, 2).unwrap();

    row * 8 + column
}

pub fn part1() -> i32 {
    let file = fs::read_to_string(INPUT_FILE).expect("Failed reading puzzle file");

    file.lines().map(generate_seat_id).fold(
        0,
        |current_max, id| if id > current_max { id } else { current_max },
    )
}

pub fn part2() -> i32 {
    let file = fs::read_to_string(INPUT_FILE).expect("Failed reading puzzle file");

    let mut seat_ids: Vec<i32> = file.lines().map(generate_seat_id).collect();
    seat_ids.sort();

    let mut index = seat_ids[0];
    let max = seat_ids[seat_ids.len() - 1];

    while index < max {
        if !seat_ids.contains(&index) {
            break;
        }
        index += 1;
    }

    index
}
