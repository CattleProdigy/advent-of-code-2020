use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;

fn validate(list: &Vec<i64>, preamble_size: usize) -> Option<i64> {
    let mut rolling_pairs = HashMap::<i64, Vec<(i64, i64)>>::new();
    let preamble: Vec<_> = list.iter().cloned().take(preamble_size).collect();
    for (i, x) in preamble.iter().cloned().enumerate() {
        for (_, y) in preamble.iter().cloned().skip(i).enumerate() {
            if x != y {
                let sum = x + y;
                let pair = if x > y { (x, y) } else { (y, x) };
                rolling_pairs.entry(sum).or_insert(Vec::new()).push(pair);
            }
        }
    }

    let leading_iter = list.iter().skip(preamble_size);
    let lagging_iter = list.iter();
    for (lead, lag) in leading_iter.zip(lagging_iter) {
        if !rolling_pairs.contains_key(lead) {
            return Some(*lead);
        }
        // (1) find each entry that contains the lagging value and
        // remove a pair containing it.
        for (_, v) in rolling_pairs.iter_mut() {
            match v
                .iter()
                .enumerate()
                .find(|(_, (x, y))| *x == *lag || *y == *lag)
            {
                Some((i, _)) => {
                    v.swap_remove(i);
                }
                None => (),
            };
        }

        // (2) remove empty pairs
        rolling_pairs.retain(|_, v| !v.is_empty());

        // (3) get list of remaining individual numbers
        let mut cur_values = HashSet::<i64>::new();
        for pairs in rolling_pairs.values() {
            for (x, y) in pairs.iter() {
                cur_values.insert(*x);
                cur_values.insert(*y);
            }
        }
        for i in cur_values.iter() {
            if *i != *lead {
                let sum = i + lead;
                let pair = if i > lead { (*i, *lead) } else { (*lead, *i) };
                rolling_pairs.entry(sum).or_insert(Vec::new()).push(pair);
            }
        }
    }

    None
}

fn find_contiguous(list: &Vec<i64>, target: i64) -> Option<(i64, i64)> {
    for window_size in 2..list.len() {
        match list
            .windows(window_size)
            .find(|x| x.iter().sum::<i64>() == target)
        {
            Some(matching_window) => {
                return Some((
                    *matching_window.iter().min().unwrap(),
                    *matching_window.iter().max().unwrap(),
                ))
            }
            None => {}
        }
    }
    None
}

fn parse(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|x| str::parse::<i64>(x).unwrap())
        .collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("We need exactly one arguemnt, got: {}", args.len() - 1);
    }

    let file_contents = fs::read_to_string(&args[1]).expect("couldn't read the file");
    let list = parse(&file_contents);
    let part1 = validate(&list, 25);

    println!("part1: {:?}", part1);

    let part2 = find_contiguous(&list, part1.unwrap());
    println!(
        "part2: {:?}: {}",
        part2,
        part2.unwrap().0 + part2.unwrap().1
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let mut base_example: Vec<i64> = (1..26).collect();
        {
            let mut ex = base_example.to_vec();
            ex.push(26);
            let res = validate(&ex, 25);
            assert!(res.is_none());
        }
        {
            let mut ex = base_example.to_vec();
            ex.push(49);
            let res = validate(&ex, 25);
            assert!(res.is_none());
        }
        {
            let mut ex = base_example.to_vec();
            ex.push(100);
            let res = validate(&ex, 25);
            assert!(res.is_some());
            assert_eq!(res.unwrap(), 100);
        }
        {
            let mut ex = base_example.to_vec();
            ex.push(50);
            let res = validate(&ex, 25);
            assert!(res.is_some());
            assert_eq!(res.unwrap(), 50);
        }
    }

    #[test]
    fn test1() {
        let example: Vec<i64> = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];
        let res = validate(&example, 5);
        assert!(res.is_some());
        assert_eq!(res.unwrap(), 127);
    }
}
