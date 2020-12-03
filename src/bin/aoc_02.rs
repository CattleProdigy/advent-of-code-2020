use regex::Regex;
use std::env;
use std::fs;

#[derive(Debug)]
struct FormatPart1 {
    min: usize,
    max: usize,
    letter: char,
}
#[derive(Debug)]
struct FormatPart2 {
    pos1: usize,
    pos2: usize,
    letter: char,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("We need exactly one argument, got: {}", args.len() - 1)
    }

    let file_contents = fs::read_to_string(&args[1]).expect("couldn't read the file");
    let re = Regex::new(r"([0-9]+)-([0-9]+) ([a-z]): ([a-z]+)").unwrap();
    let num_valid_part1 = re
        .captures_iter(&file_contents)
        .map(|caps| {
            (
                FormatPart1 {
                    min: caps[1].parse::<usize>().unwrap(),
                    max: caps[2].parse::<usize>().unwrap(),
                    letter: caps[3].parse::<char>().unwrap(),
                },
                caps[4].to_string(),
            )
        })
        .filter(|(fmt, pw)| {
            let char_count = pw.chars().filter(|&x| x == fmt.letter).count();
            fmt.min <= char_count && char_count <= fmt.max
        })
        .count();

    println!("Part 1 : {}", num_valid_part1);

    let num_valid_part2 = re
        .captures_iter(&file_contents)
        .map(|caps| {
            (
                FormatPart2 {
                    pos1: caps[1].parse::<usize>().unwrap(),
                    pos2: caps[2].parse::<usize>().unwrap(),
                    letter: caps[3].parse::<char>().unwrap(),
                },
                caps[4].to_string(),
            )
        })
        .filter(|(fmt, pw)| {
            let char_at_pos1 = pw.chars().nth(fmt.pos1 - 1).unwrap() == fmt.letter;
            let char_at_pos2 = pw.chars().nth(fmt.pos2 - 1).unwrap() == fmt.letter;
            char_at_pos1 != char_at_pos2
        })
        .count();

    println!("Part 2 : {}", num_valid_part2);
}
