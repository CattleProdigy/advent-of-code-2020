use std::collections::HashSet;
use std::env;
use std::fs;

fn count_part1(file: &str) -> usize {
    file.split("\n\n")
        .map(|x| {
            x.chars()
                .filter(|x| x.is_alphabetic())
                .collect::<HashSet<char>>()
        })
        .collect::<Vec<_>>()
        .iter()
        .map(|x| x.len())
        .sum()
}

fn count_part2(file: &str) -> usize {
    file.split("\n\n")
        .map(|x| {
            let mut itr = x.split("\n");
            let mut first = itr.next().unwrap().chars().collect::<HashSet<char>>();
            itr.filter(|x| !x.is_empty())
                .map(|x| x.chars().collect::<HashSet<char>>())
                .fold(&mut first, |acc, x| {
                    *acc = acc.intersection(&x).map(|x| *x).collect::<HashSet<char>>();
                    acc
                })
                .clone()
        })
        .collect::<Vec<_>>()
        .iter()
        .map(|x| x.len())
        .sum()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("We need exactly one arguemnt, got: {}", args.len() - 1);
    }

    let file_contents = fs::read_to_string(&args[1]).expect("couldn't read the file");

    println!("part 1 {}", count_part1(&file_contents));
    println!("part 2 {}", count_part2(&file_contents));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let example = r#"abc

a
b
c

ab
ac

a
a
a
a

b"#;

        assert_eq!(count_part1(example), 11);
        assert_eq!(count_part2(example), 6);
    }
}
