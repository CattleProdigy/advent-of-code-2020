use regex::Regex;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::env;
use std::fs;

fn parse(file: &str) -> (Vec<Vec<(usize, usize)>>, usize) {
    let mut bag_names: Vec<String> = Vec::new();
    let mut get_id = |x: &str| -> usize {
        match bag_names.iter().enumerate().find(|(_, b)| **b == *x) {
            Some((idx, _)) => idx,
            None => {
                bag_names.push(x.to_string());
                bag_names.len() - 1
            }
        }
    };

    let re = Regex::new(r"(\d+) ([\w|\s]+) bags?").unwrap();

    let rules_map: BTreeMap<_, _> = file
        .lines()
        .map(|x| {
            static DELIM1: &str = " bags contain ";
            let idx = x.find(DELIM1).unwrap();
            let beg_slice = &x[..idx];
            let bag_id = get_id(beg_slice);
            let end_slice = &x[(idx + DELIM1.len())..];
            let rule = end_slice
                .split(",")
                .filter(|x| !x.is_empty())
                .filter(|x| *x != "no other bags.")
                .map(|x| {
                    let caps = re.captures(&x).unwrap();
                    let rule_bag_id = get_id(&caps[2]);
                    let num = caps[1].parse::<usize>().unwrap();
                    (num, rule_bag_id)
                })
                .collect::<Vec<_>>();
            (bag_id, rule)
        })
        .collect();

    let rules: Vec<Vec<(usize, usize)>> = rules_map.values().map(|x| x.clone()).collect();

    for (i, bn) in bag_names.iter().enumerate() {
        println!("{}: {}", i, bn);
    }

    let shiny_id = bag_names
        .iter()
        .enumerate()
        .find(|(_, s)| *s == "shiny gold")
        .unwrap()
        .0;

    (rules, shiny_id)
}

fn get_containers(rules: &Vec<Vec<(usize, usize)>>, id: usize) -> Vec<usize> {
    rules
        .iter()
        .enumerate()
        .filter(|(_, x)| match x.iter().find(|(_, bid)| *bid == id) {
            Some(_) => true,
            None => false,
        })
        .map(|(i, _)| i)
        .collect()
}

fn part1(rules: &Vec<Vec<(usize, usize)>>, shiny_id: usize) -> usize {
    let mut queue: VecDeque<usize> = vec![shiny_id].into_iter().collect();
    let mut colors = HashSet::<usize>::new();

    while !queue.is_empty() {
        let cur_bag = queue.pop_front().unwrap();
        let cur_containers = get_containers(rules, cur_bag);
        for c in cur_containers.iter() {
            queue.push_back(*c);
            colors.insert(*c);
        }
    }

    colors.len()
}

fn part2(rules: &Vec<Vec<(usize, usize)>>, shiny_id: usize) -> usize {
    let mut queue: VecDeque<(usize, usize)> = vec![(shiny_id, 1)].into_iter().collect();

    let mut total: usize = 0;
    while !queue.is_empty() {
        let (cur_bag, parent_scalar) = queue.pop_front().unwrap();
        let cur_containees = &rules[cur_bag];
        for (n, bid) in cur_containees.iter() {
            queue.push_back((*bid, n * parent_scalar));
            total += n * parent_scalar;
        }
    }

    total
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("We need exactly one arguemnt, got: {}", args.len() - 1);
    }

    let file_contents = fs::read_to_string(&args[1]).expect("couldn't read the file");

    let (r, s) = parse(&file_contents);
    println!("part 1: {}", part1(&r, s));
    println!("part 2: {}", part2(&r, s));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let example = r#"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."#;

        let (r, s) = parse(example);

        assert_eq!(part1(&r, s), 4);
        assert_eq!(part2(&r, s), 32);
    }

    #[test]
    fn test_part2() {
        let example = r#"shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags."#;

        let (r, s) = parse(example);

        assert_eq!(part2(&r, s), 126);
    }
}
