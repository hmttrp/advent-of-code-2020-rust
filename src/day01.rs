use std::fs;

fn read_puzzle() -> String {
    fs::read_to_string("./input/day01.txt").expect("Failed reading puzzle file")
}

pub fn part1() -> i32 {
    let input = read_puzzle();
    part1_solution(&input)
}

fn part1_solution(input: &str) -> i32 {
    let numbers: Vec<i32> = input.lines().map(|x| x.parse().unwrap()).collect();

    for x in &numbers {
        for y in &numbers {
            if x + y == 2020 {
                return x * y;
            }
        }
    }

    panic!("should never reach this point");
}

pub fn part2() -> i32 {
    let input = read_puzzle();
    part2_solution(&input)
}

fn part2_solution(input: &str) -> i32 {
    let numbers: Vec<i32> = input.lines().map(|x| x.parse().unwrap()).collect();

    for x in &numbers {
        for y in &numbers {
            for z in &numbers {
                if x + y + z == 2020 {
                    return x * y * z;
                }
            }
        }
    }

    panic!("should never reach this point");
}

#[cfg(test)]
mod tests {
    const EXAMPLE_INPUT: &str = r"1721
979
366
299
675
1456";

    #[test]
    fn part_one_example() {
        assert_eq!(super::part1_solution(EXAMPLE_INPUT), 514579);
    }

    #[test]
    fn part_one() {
        assert_eq!(super::part1(), 960075);
    }

    #[test]
    fn part_two_example() {
        assert_eq!(super::part2_solution(EXAMPLE_INPUT), 241861950);
    }

    #[test]
    fn part_two() {
        assert_eq!(super::part2(), 212900130);
    }
}
