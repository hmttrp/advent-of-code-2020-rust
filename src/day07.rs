use std::fs;

const INPUT_FILE: &str = "./input/day07.txt";
// const INPUT_FILE: &str = "./input/day07_example.txt";

const BAG_COLOR: &str = "shiny gold";

#[derive(Debug)]
struct Bag {
    color: String,
    content: Vec<(usize, String)>,
}

impl Bag {
    fn can_hold(&self, bag_color: &str) -> bool {
        self.content.iter().find(|c| c.1 == bag_color).is_some()
    }
}

impl PartialEq for Bag {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color
    }
}

fn parse_rules(rules_string: &str) -> Vec<Bag> {
    rules_string
        .lines()
        .map(|rule| {
            let splitted_rule: Vec<&str> =
                rule[0..rule.len() - 1].split(" bags contain ").collect();

            let content: Vec<(usize, String)> = match splitted_rule[1] {
                "no other bags" => vec![],
                content_rule => content_rule
                    .split(", ")
                    .map(|bag_content_rule| {
                        let (count_as_string, bag) = bag_content_rule.split_at(2);
                        let count: usize = count_as_string.trim().parse().unwrap();

                        let bag_color = bag
                            .split_whitespace()
                            .take(2)
                            .collect::<Vec<&str>>()
                            .join(" ")
                            .to_string();

                        (count, bag_color)
                    })
                    .collect(),
            };

            Bag {
                color: splitted_rule[0].to_string(),
                content,
            }
        })
        .collect()
}

fn find_bags_containing_bag<'a>(bags: &'a Vec<Bag>, bag_color: &str) -> Vec<&'a Bag> {
    let found_bags: Vec<&Bag> = bags.iter().filter(|bag| bag.can_hold(bag_color)).collect();

    let more_bags: Vec<&Bag> = found_bags
        .iter()
        .flat_map(|bag| find_bags_containing_bag(bags, &bag.color))
        .collect();

    let mut bags: Vec<&Bag> = vec![];

    for bag in found_bags {
        bags.push(bag);
    }

    for bag in more_bags {
        if !bags.contains(&bag) {
            bags.push(bag)
        }
    }

    bags
}

pub fn part1() -> usize {
    let file = fs::read_to_string(INPUT_FILE).expect("Failed reading puzzle file");
    let rules: Vec<Bag> = parse_rules(&file);
    let found = find_bags_containing_bag(&rules, &BAG_COLOR);
    found.len()
}

fn bag_count(bags: &Vec<Bag>, bag_color: &str) -> usize {
    let maybe_bag = bags.iter().find(|b| b.color == bag_color);

    match maybe_bag {
        None => 0,
        Some(bag) => bag
            .content
            .iter()
            .map(|c| c.0 + c.0 * bag_count(&bags, &c.1))
            .sum::<usize>(),
    }
}

pub fn part2() -> usize {
    let file = fs::read_to_string(INPUT_FILE).expect("Failed reading puzzle file");
    let rules: Vec<Bag> = parse_rules(&file);

    bag_count(&rules, BAG_COLOR)
}
