use std::collections::HashMap;
use std::env;
use std::fs;

fn preprocess_jolts(jolts: &[i64]) -> Vec<i64> {
    let mut sorted = jolts.to_vec();
    sorted.sort();
    sorted.insert(0, 0);
    sorted.push(sorted.iter().last().unwrap() + 3);
    sorted
}

fn part1(jolts: &[i64]) -> (i64, i64, i64) {
    let sorted = preprocess_jolts(jolts);
    let mut jolt_diffs = sorted.windows(2).map(|x| x[1] - x[0]).collect::<Vec<i64>>();
    jolt_diffs.sort();

    let ones = jolt_diffs
        .iter()
        .skip_while(|&&x| x != 1)
        .take_while(|&&x| x == 1)
        .count() as i64;
    let threes = jolt_diffs
        .iter()
        .skip_while(|&&x| x != 3)
        .take_while(|&&x| x == 3)
        .count() as i64;

    (ones, threes, ones * threes)
}

fn part2(jolts: &[i64]) -> usize {
    let sorted = preprocess_jolts(jolts);
    let target_val = *sorted.last().unwrap();

    // memorizer maps a value to the number of paths to
    // the target jolt value, such that first value has
    // the total number of paths by the end
    let mut memoizer = HashMap::<i64, usize>::new();
    memoizer.insert(target_val, 1);

    // DFS-ish with memoizing
    let mut search_stack = Vec::<i64>::new();
    search_stack.push(0);
    while !search_stack.is_empty() {
        let cur_val = search_stack.pop().unwrap();
        if memoizer.contains_key(&cur_val) {
            panic!("Don't push a value if you haven't pushed the children first");
        }

        // Could push the index instead of searching but whatever
        let idx = sorted.binary_search(&cur_val).unwrap();
        let children = sorted
            .iter()
            .skip(idx + 1)
            .take_while(|&x| (x - cur_val) <= 3)
            .collect::<Vec<_>>();

        // If all the children are memoized then just sum them up
        if children.iter().all(|&x| memoizer.contains_key(&x)) {
            let children_count = children
                .iter()
                .map(|&x| memoizer.get(&x).unwrap())
                .sum::<usize>();
            memoizer.insert(cur_val, children_count);
        } else {
            // push the current value again so we can revisit after
            // the children have been processed, at that time we should
            // hit the "true" branch". This is the sort of thing that's
            // more elegant with recursion.
            search_stack.push(cur_val);

            // Visit any outstanding children
            for &i in children.iter().filter(|&x| !memoizer.contains_key(x)) {
                search_stack.push(*i);
            }
        }
    }

    *memoizer.get(&0).unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("We need exactly one arguemnt, got: {}", args.len() - 1);
    }

    let file_contents = fs::read_to_string(&args[1]).expect("couldn't read the file");
    let jolts: Vec<i64> = file_contents
        .lines()
        .map(|x| str::parse::<i64>(x).unwrap())
        .collect();

    let (ones, threes, p1) = part1(&jolts);
    println!("ones: {}, threes: {}, p1: {}", ones, threes, p1);
    println!("p2: {}", part2(&jolts));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let example: Vec<i64> = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        let (ones, threes, p1) = part1(&example);
        assert_eq!(ones, 7);
        assert_eq!(threes, 5);
        assert_eq!(p1, 35);

        assert_eq!(part2(&example), 8);
    }
    #[test]
    fn test1() {
        let example: Vec<i64> = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];
        let (ones, threes, p1) = part1(&example);
        assert_eq!(ones, 22);
        assert_eq!(threes, 10);
        assert_eq!(p1, 220);

        assert_eq!(part2(&example), 19208);
    }
}
