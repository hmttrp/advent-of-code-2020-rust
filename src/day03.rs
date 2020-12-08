use std::fs;

const INPUT_FILE: &str = "./input/day03.txt";
// const INPUT_FILE: &str = "./input/day03_example.txt";

fn get_puzzle() -> Vec<Vec<char>> {
    let file = fs::read_to_string(INPUT_FILE).expect("Failed reading puzzle file");
    file.lines().map(|s| s.chars().collect()).collect()
}

fn check_slope(map: &Vec<Vec<char>>, step_right: usize, step_down: usize) -> i64 {
    let map_width = map[0].len();
    let mut trees = 0;
    let mut pos_x = 0;
    let mut pos_y = 0;

    while pos_y < map.len() {
        let current_position = map[pos_y][pos_x];

        if current_position == '#' {
            trees += 1;
        }

        pos_x = (pos_x + step_right) % map_width;
        pos_y += step_down;
    }

    trees
}

pub fn part1() -> i64 {
    let map = get_puzzle();
    check_slope(&map, 3, 1)
}

pub fn part2() -> i64 {
    let map = get_puzzle();

    check_slope(&map, 1, 1)
        * check_slope(&map, 3, 1)
        * check_slope(&map, 5, 1)
        * check_slope(&map, 7, 1)
        * check_slope(&map, 1, 2)
}
