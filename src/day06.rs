use std::collections::HashSet;
use std::fs;

const INPUT_FILE: &str = "./input/day06.txt";

pub fn part1() -> usize {
    let file = fs::read_to_string(INPUT_FILE).expect("Failed reading puzzle file");

    file.split("\n\n")
        .map(|x| {
            x.chars()
                .filter(|c| c.is_ascii_alphabetic())
                .collect::<HashSet<_>>()
                .len()
        })
        .sum::<usize>()
}

fn all_answers() -> HashSet<char> {
    ('a'..='z').collect()
}

pub fn part2() -> usize {
    let file = fs::read_to_string(INPUT_FILE).expect("Failed reading puzzle file");

    file.split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|line| line.chars().collect::<HashSet<_>>())
                .fold(all_answers(), |acc, answers| {
                    acc.intersection(&answers).cloned().collect()
                })
                .len()
        })
        .sum::<usize>()
}
