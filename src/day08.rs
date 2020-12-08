use std::collections::HashSet;
use std::convert::TryFrom;
use std::fs;

fn read_puzzle() -> String {
    fs::read_to_string("./input/day08.txt").expect("Failed reading puzzle file")
}

/*
acc
increases or decreases a single global value called the accumulator by the value
given in the argument. For example, acc +7 would increase the accumulator by 7.
The accumulator starts at 0. After an acc instruction, the instruction immediately
below it is executed next.

jmp
jumps to a new instruction relative to itself. The next instruction to execute is
found using the argument as an offset from the jmp instruction; for example, jmp
+2 would skip the next instruction, jmp +1 would continue to the instruction
immediately below it, and jmp -20 would cause the instruction 20 lines above to be
executed next.

nop
stands for No OPeration - it does nothing.
The instruction immediately below it is executed next.
*/

#[derive(Debug, Clone)]
enum Command {
    Accumulate(i32),
    Jump(i32),
    NoOperation(i32),
}

impl Command {
    fn from(string: &str) -> Command {
        let command_parts: Vec<&str> = string.split_whitespace().collect();
        let command = command_parts[0];
        let sign: i32 = if command_parts[1].split_at(1).0 == "+" {
            1
        } else {
            -1
        };
        let argument: i32 = sign * command_parts[1].split_at(1).1.parse::<i32>().unwrap();

        match command {
            "acc" => Command::Accumulate(argument),
            "jmp" => Command::Jump(argument),
            "nop" => Command::NoOperation(argument),
            _ => panic!("unknown operation"),
        }
    }
}

fn execute_programm(program: &Vec<Command>) -> Result<i32, i32> {
    let mut acc: i32 = 0;
    let mut index: i32 = 0;

    let mut executed_commands: HashSet<i32> = HashSet::new();

    loop {
        if executed_commands.contains(&index) {
            println!("Loop detected. Stopping with acc: {}", acc);
            break Err(acc);
        }
        executed_commands.insert(index);

        let usize_index = usize::try_from(index).unwrap();

        if usize_index >= program.len() {
            println!("Finished with acc: {}", acc);
            break Ok(acc);
        }

        let command = &program[usize_index];

        match command {
            Command::Accumulate(value) => {
                acc += value;
                index += 1;
            }
            Command::Jump(value) => {
                index += value;
            }
            Command::NoOperation(_) => {
                index += 1;
            }
        }
    }
}

pub fn part1() -> i32 {
    let input = read_puzzle();
    part1_solution(&input)
}

fn part1_solution(input: &str) -> i32 {
    let program: Vec<Command> = input.lines().map(Command::from).collect();
    match execute_programm(&program) {
        Ok(n) => n,
        Err(n) => n,
    }
}

pub fn part2() -> i32 {
    let input = read_puzzle();
    part2_solution(&input)
}

fn part2_solution(input: &str) -> i32 {
    let program: Vec<Command> = input.lines().map(Command::from).collect();

    let mut result = 0;

    for (i, command) in program.iter().enumerate() {
        // println!("{} {:?}", i, command);

        match command {
            Command::Jump(v) => {
                let mut modified_program = program.clone();
                let new_command = Command::NoOperation(*v);
                modified_program[i] = new_command;
                let res = execute_programm(&modified_program) ;
                if res.is_ok() {
                    result = res.unwrap();
                    break;
                }
            }
            Command::NoOperation(v) => {
                let mut modified_program = program.clone();
                let new_command = Command::Jump(*v);
                modified_program[i] = new_command;
                let res = execute_programm(&modified_program) ;
                if res.is_ok() {
                    result = res.unwrap();
                    break;
                }
            }
            _ => println!("nothing to do here"),
        }
    }

    result
}

#[cfg(test)]
mod tests {
    const EXAMPLE_INPUT: &str = r"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    #[test]
    fn part_one_example() {
        assert_eq!(super::part1_solution(EXAMPLE_INPUT), 5);
    }

    #[test]
    fn part_one() {
        assert_eq!(super::part1(), 1179);
    }

    #[test]
    fn part_two_example() {
        assert_eq!(super::part2_solution(EXAMPLE_INPUT), 8);
    }

    #[test]
    fn part_two() {
        assert_eq!(super::part2(), 1089);
    }
}
