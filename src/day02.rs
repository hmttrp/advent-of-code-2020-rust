use std::convert::TryInto;
use std::fs;

const INPUT_FILE: &str = "./input/day02.txt";

fn str_to_string(s: &str) -> String {
    s.parse().unwrap()
}

fn get_puzzle() -> Vec<String> {
    let file = fs::read_to_string(INPUT_FILE).expect("Failed reading puzzle file");
    file.lines().map(str_to_string).collect()
}

pub fn part1() -> i32 {
    let lines = get_puzzle();

    let mut valid_password_count = 0;

    for line in lines {
        let splitted: Vec<&str> = line
            .split(": ")
            .flat_map(|s: &str| s.split_whitespace())
            .flat_map(|s: &str| s.split('-'))
            .collect();

        let min: i32 = splitted[0].parse().unwrap();
        let max: i32 = splitted[1].parse().unwrap();
        let character = splitted[2];
        let password = splitted[3];

        let character_count: i32 = password.matches(character).count().try_into().unwrap();

        if character_count >= min && character_count <= max {
            valid_password_count = valid_password_count + 1
        }
    }

    valid_password_count
}

pub fn part2() -> i32 {
    let lines = get_puzzle();

    let mut valid_password_count = 0;

    for line in lines {
        let splitted: Vec<&str> = line
            .split(": ")
            .flat_map(|s: &str| s.split(' '))
            .flat_map(|s: &str| s.split('-'))
            .collect();

        let first_index: usize = splitted[0].parse().map(|x: usize| x - 1).unwrap();
        let second_index: usize = splitted[1].parse().map(|x: usize| x - 1).unwrap();
        let character = splitted[2];
        let password = splitted[3];

        let first_character_is_valid =
            password.chars().nth(first_index).unwrap().to_string() == character;
        let second_character_is_valid =
            password.chars().nth(second_index).unwrap().to_string() == character;

        if first_character_is_valid ^ second_character_is_valid {
            valid_password_count = valid_password_count + 1
        }
    }

    valid_password_count
}
