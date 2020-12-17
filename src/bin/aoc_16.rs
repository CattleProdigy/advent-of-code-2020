use regex::Regex;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::env;
use std::fs;

#[derive(Debug)]
struct Notes {
    rules: HashMap<String, ((i64, i64), (i64, i64))>,
    your_ticket: Vec<i64>,
    nearby_tickets: Vec<Vec<i64>>,
}

fn eval_rule(val: i64, range: &((i64, i64), (i64, i64))) -> bool {
    (val >= range.0 .0 && val <= range.0 .1) || (val >= range.1 .0 && val <= range.1 .1)
}

fn invalid_ticket_vals(
    ticket: &Vec<i64>,
    rules: &HashMap<String, ((i64, i64), (i64, i64))>,
) -> Vec<i64> {
    ticket
        .iter()
        .filter_map(|x| {
            if !rules.values().any(|r| eval_rule(*x, r)) {
                Some(*x)
            } else {
                None
            }
        })
        .collect()
}

fn ticket_scan_error_rate(notes: &Notes) -> i64 {
    notes
        .nearby_tickets
        .iter()
        .map(|x| invalid_ticket_vals(x, &notes.rules))
        .flatten()
        .sum()
}

fn p2_notes_filter(notes: &Notes) -> Notes {
    let filtered_nearby_tickets = notes
        .nearby_tickets
        .iter()
        .filter(|x| invalid_ticket_vals(x, &notes.rules).is_empty())
        .cloned()
        .collect::<Vec<Vec<i64>>>();

    Notes {
        rules: notes.rules.clone(),
        your_ticket: notes.your_ticket.to_vec(),
        nearby_tickets: filtered_nearby_tickets,
    }
}

fn parse(input: &str) -> Notes {
    let mut iter = input.split("\n\n").filter(|x| !x.is_empty());

    let re = Regex::new(r"(.*): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();

    let rules: HashMap<String, ((i64, i64), (i64, i64))> = iter
        .next()
        .unwrap()
        .lines()
        .map(|x| {
            let caps = re.captures(x).unwrap();
            let label = caps.get(1).unwrap().as_str().to_string();

            let num_1 = caps.get(2).unwrap().as_str().parse::<i64>().unwrap();
            let num_2 = caps.get(3).unwrap().as_str().parse::<i64>().unwrap();
            let num_3 = caps.get(4).unwrap().as_str().parse::<i64>().unwrap();
            let num_4 = caps.get(5).unwrap().as_str().parse::<i64>().unwrap();

            (label, ((num_1, num_2), (num_3, num_4)))
        })
        .collect();

    let your_ticket = iter
        .next()
        .unwrap()
        .lines()
        .nth(1)
        .unwrap()
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let nearby_tickets = iter
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(|l| {
            l.split(',')
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>();

    Notes {
        rules: rules,
        your_ticket: your_ticket,
        nearby_tickets: nearby_tickets,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("We need exactly one arguemnt, got: {}", args.len() - 1);
    }

    let file_contents = fs::read_to_string(&args[1]).expect("couldn't read the file");
    let notes = parse(&file_contents);

    let error_rate = ticket_scan_error_rate(&notes);
    println!("p1: {}", error_rate);

    let filtered_notes = p2_notes_filter(&notes);

    let tickets: Vec<Vec<i64>> = filtered_notes
        .nearby_tickets
        .iter()
        .cloned()
        .chain(std::iter::once(filtered_notes.your_ticket))
        .collect();

    let ticket_len = tickets[0].len();

    let rules = filtered_notes.rules;

    let mut possible_rules: Vec<(i64, Vec<&str>)> = Vec::new();
    for i in 0..ticket_len {
        let mut pr: Vec<&str> = Vec::new();

        for (k, v) in rules.iter() {
            if tickets.iter().all(|t| {
                let val = t[i];
                eval_rule(val, v)
            }) {
                pr.push(k);
            }
        }
        pr.sort_by(|a, b| a.len().partial_cmp(&b.len()).unwrap());
        possible_rules.push((i as i64, pr));
    }
    possible_rules.sort_by(|a, b| a.1.len().partial_cmp(&b.1.len()).unwrap());

    let mut shrinking_rules = possible_rules.to_vec();
    let mut result: BTreeMap<&str, i64> = BTreeMap::new();
    while result.len() < ticket_len {
        let (i, pr) = shrinking_rules.iter().find(|(_, v)| v.len() == 1).unwrap();
        let label: &str = pr[0];
        result.insert(label, *i);
        for (_, v) in shrinking_rules.iter_mut() {
            v.retain(|&x| x != label);
        }
    }

    let product = result
        .iter()
        .filter_map(|(k, &v)| {
            if k.starts_with("departure") {
                Some(notes.your_ticket[v as usize])
            } else {
                None
            }
        })
        .product::<i64>();

    println!("p2: {}", product);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let example = r#"class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12"#;

        let notes = parse(&example);
        println!("{:?}", notes);

        let error_rate = ticket_scan_error_rate(&notes);
        assert_eq!(error_rate, 71);

        let filtered = p2_notes_filter(&notes);
        let filter_truth: Vec<Vec<i64>> = vec![vec![7, 3, 47]];
        assert_eq!(filtered.nearby_tickets, filter_truth);
    }
}
