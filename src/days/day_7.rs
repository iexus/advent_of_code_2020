use std::collections::HashMap;
use std::collections::HashSet;

use regex::Regex;

pub fn call(puzzle_input: String) {
    do_the_thing(puzzle_input);
}

fn do_the_thing(puzzle_input: String) -> (usize, usize) {
    let mut downward_map = HashMap::<String, Vec<(String, u8)>>::new();
    let mut upward_map = HashMap::<String, HashSet<String>>::new();

    let bag_matcher = Regex::new(r"(?P<count>(\d+)) (?P<bag_name>([\w|\s]+)) bags?[,.]").unwrap();

    puzzle_input.lines().for_each(|line| {
        let parts: Vec<&str> = line.split("contain").collect();

        let raw_container = parts[0];
        let without_bags = raw_container.replace("bags", "").replace("bag", "");
        let parent_bag = without_bags.trim();

        let mut contents = Vec::<(String, u8)>::new();
        for cap in bag_matcher.captures_iter(parts[1]) {
            // add each one to the list
            let bag_name = cap.name("bag_name").unwrap().as_str();
            let bag_count: u8 = cap.name("count").unwrap().as_str().parse().unwrap();

            // build the list for going down
            contents.push((bag_name.to_string(), bag_count));

            // add an entry for going up (reverse of above)
            let parents = upward_map.entry(bag_name.to_string()).or_insert(HashSet::<String>::new());
            (*parents).insert(parent_bag.to_string());
        }

        downward_map.insert(parent_bag.to_string(), contents);
    });

    let result_a = calculate_bags_that_could_hold_gold(upward_map);
    println!("Result A: {}", result_a);

    let result_b = calculate_bags_that_gold_could_hold(downward_map);
    println!("Result B: {}", result_b);

    (result_a, result_b)
}

fn calculate_bags_that_could_hold_gold(upward_map: HashMap<String, HashSet<String>>) -> usize {
    let mut unique_bags_that_could_contain_gold = HashSet::<String>::new();
    crawl_bag_parents_for("shiny gold", &upward_map, &mut unique_bags_that_could_contain_gold);

    unique_bags_that_could_contain_gold.len()
}

fn calculate_bags_that_gold_could_hold(downward_map: HashMap<String, Vec<(String, u8)>>) -> usize {
    crawl_bag_children_for("shiny gold", &downward_map)
}

fn crawl_bag_children_for(bag_name: &str, lookdown: &HashMap<String, Vec<(String, u8)>>) -> usize {
    let mut count = 0;
    if bag_name != "shiny gold" {
        count += 1;
    }

    // Count how many children we can hold
    match lookdown.get(bag_name) {
        None => (),
        Some(contents) => {
            let child_count = contents.iter().fold(0, |accumulator, bag| {
                let child_contents = crawl_bag_children_for(&bag.0, lookdown);
                accumulator + (child_contents * bag.1 as usize)
            });

            count += child_count;
        }
    }

    count
}

fn crawl_bag_parents_for(bag_name: &str, lookup: &HashMap<String, HashSet<String>>, acc: &mut HashSet<String>) {
    if bag_name != "shiny gold" {
        let copy_of_name = bag_name.to_string();
        acc.insert(copy_of_name);
    }

    match lookup.get(bag_name) {
        None => (),
        Some(contents) => {
            contents.iter().for_each(|name| {
                crawl_bag_parents_for(name, lookup, acc)
            });
        },
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_the_right_number_for_a_basic_example() {
        let input = String::from("light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.");

        let result = do_the_thing(input);
        assert_eq!(result.0, 4);
    }

    #[test]
    fn it_returns_the_number_of_bags_that_gold_could_gold() {
        let input = String::from("shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.");

        let result = do_the_thing(input);
        assert_eq!(result.1, 126);
    }
}


