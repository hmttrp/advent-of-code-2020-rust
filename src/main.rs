use std::collections::HashSet;
use std::env;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
// mod day09;
// mod day10;
// mod day11;
// mod day12;
// mod day13;
// mod day14;
// mod day15;
// mod day16;
// mod day17;
// mod day18;
// mod day19;
// mod day20;
// mod day21;
// mod day22;
// mod day23;
// mod day24;

fn print_banner() {
    println!(r" _____   _             _          ___    _____       _        ___ ___ ___ ___ ");
    println!(r"|  _  |_| |_ _ ___ ___| |_    ___|  _|  |     |___ _| |___   |_  |   |_  |   |");
    println!(r"|     | . | | | -_|   |  _|  | . |  _|  |   --| . | . | -_|  |  _| | |  _| | |");
    println!(r"|__|__|___|\_/|___|_|_|_|    |___|_|    |_____|___|___|___|  |___|___|___|___|");
    println!();
}

fn main() {
    let args: HashSet<_> = env::args().skip(1).collect();
    print_banner();

    if args.is_empty() || args.contains("1") || args.contains("01") {
        println!("-- Day 1 --");
        println!("Part 1: {:?}", day01::part1());
        println!("Part 2: {:?}", day01::part2());
        println!();
    }

    if args.is_empty() || args.contains("2") || args.contains("02") {
        println!("-- Day 2 --");
        println!("Part 1: {:?}", day02::part1());
        println!("Part 2: {:?}", day02::part2());
        println!();
    }

    if args.is_empty() || args.contains("3") || args.contains("03") {
        println!("-- Day 3 --");
        println!("Part 1: {:?}", day03::part1());
        println!("Part 2: {:?}", day03::part2());
        println!();
    }

    if args.is_empty() || args.contains("4") || args.contains("04") {
        println!("-- Day 4 --");
        println!("Part 1: {:?}", day04::part1());
        println!("Part 2: {:?}", day04::part2());
        println!();
    }

    if args.is_empty() || args.contains("5") || args.contains("05") {
        println!("-- Day 5 --");
        println!("Part 1: {:?}", day05::part1());
        println!("Part 2: {:?}", day05::part2());
        println!();
    }

    if args.is_empty() || args.contains("6") || args.contains("06") {
        println!("-- Day 6 --");
        println!("Part 1: {:?}", day06::part1());
        println!("Part 2: {:?}", day06::part2());
        println!();
    }

    if args.is_empty() || args.contains("7") || args.contains("07") {
        println!("-- Day 7 --");
        println!("Part 1: {:?}", day07::part1());
        println!("Part 2: {:?}", day07::part2());
        println!();
    }

    if args.is_empty() || args.contains("8") || args.contains("08") {
        println!("-- Day 8 --");
        println!("Part 1: {:?}", day08::part1());
        println!("Part 2: {:?}", day08::part2());
        println!();
    }

    // if args.is_empty() || args.contains("9") || args.contains("09") {
    //     println!("-- Day 9 --");
    //     println!("Part 1: {:?}", day09::part1());
    //     println!("Part 2: {:?}", day09::part2());
    //     println!();
    // }

    // if args.is_empty() || args.contains("10") {
    //     println!("-- Day 10 --");
    //     println!("Part 1: {:?}", day10::part1());
    //     println!("Part 2: {:?}", day10::part2());
    //     println!();
    // }

    // if args.is_empty() || args.contains("11") {
    //     println!("-- Day 11 --");
    //     println!("Part 1: {:?}", day11::part1());
    //     println!("Part 2: {:?}", day11::part2());
    //     println!();
    // }

    // if args.is_empty() || args.contains("12") {
    //     println!("-- Day 12 --");
    //     println!("Part 1: {:?}", day12::part1());
    //     println!("Part 2: {:?}", day12::part2());
    //     println!();
    // }

    // if args.is_empty() || args.contains("13") {
    //     println!("-- Day 13 --");
    //     println!("Part 1: {:?}", day13::part1());
    //     println!("Part 2: {:?}", day13::part2());
    //     println!();
    // }

    // if args.is_empty() || args.contains("14") {
    //     println!("-- Day 14 --");
    //     println!("Part 1: {:?}", day14::part1());
    //     println!("Part 2: {:?}", day14::part2());
    //     println!();
    // }

    // if args.is_empty() || args.contains("15") {
    //     println!("-- Day 15 --");
    //     println!("Part 1: {:?}", day15::part1());
    //     println!("Part 2: {:?}", day15::part2());
    //     println!();
    // }

    // if args.is_empty() || args.contains("16") {
    //     println!("-- Day 16 --");
    //     println!("Part 1: {:?}", day16::part1());
    //     println!("Part 2: {:?}", day16::part2());
    //     println!();
    // }

    // if args.is_empty() || args.contains("17") {
    //     println!("-- Day 17 --");
    //     println!("Part 1: {:?}", day17::part1());
    //     println!("Part 2: {:?}", day17::part2());
    //     println!();
    // }

    // if args.is_empty() || args.contains("18") {
    //     println!("-- Day 18 --");
    //     println!("Part 1: {:?}", day18::part1());
    //     println!("Part 2: {:?}", day18::part2());
    //     println!();
    // }

    // if args.is_empty() || args.contains("19") {
    //     println!("-- Day 19 --");
    //     println!("Part 1: {:?}", day19::part1());
    //     println!("Part 2: {:?}", day19::part2());
    //     println!();
    // }

    // if args.is_empty() || args.contains("20") {
    //     println!("-- Day 20 --");
    //     println!("Part 1: {:?}", day20::part1());
    //     println!("Part 2: {:?}", day20::part2());
    //     println!();
    // }

    // if args.is_empty() || args.contains("21") {
    //     println!("-- Day 21 --");
    //     println!("Part 1: {:?}", day21::part1());
    //     println!("Part 2: {:?}", day21::part2());
    //     println!();
    // }

    // if args.is_empty() || args.contains("22") {
    //     println!("-- Day 22 --");
    //     println!("Part 1: {:?}", day22::part1());
    //     println!("Part 2: {:?}", day22::part2());
    //     println!();
    // }

    // if args.is_empty() || args.contains("23") {
    //     println!("-- Day 23 --");
    //     println!("Part 1: {:?}", day23::part1());
    //     println!("Part 2: {:?}", day23::part2());
    //     println!();
    // }

    // if args.is_empty() || args.contains("24") {
    //     println!("-- Day 24 --");
    //     println!("Part 1: {:?}", day24::part1());
    //     println!("Part 2: {:?}", day24::part2());
    //     println!();
    // }

    println!("[DONE]");
}
