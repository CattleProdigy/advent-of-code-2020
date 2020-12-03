use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!(
            "We need exactly one argument, got: {}, {:?}",
            args.len() - 1,
            args
        )
    }

    let file_contents = fs::read_to_string(&args[1]).expect("couldn't read the file");

    let numbers = file_contents
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let pair: Option<(i32, i32)> = {
        let mut maybe_pair = None;
        for (i, a) in numbers.iter().enumerate() {
            for b in numbers.iter().skip(i) {
                if a + b == 2020 {
                    maybe_pair = Some((*a, *b));
                }
            }
        }
        maybe_pair
    };

    match pair {
        None => println!("Nothing sums to 2020"),
        Some(x) => {
            println!("{0} + {1} == 2020, {0} * {1} = {2}", x.0, x.1, x.0 * x.1);
        }
    }

    let triplet: Option<(i32, i32, i32)> = {
        let mut maybe_triplet = None;
        for (i, a) in numbers.iter().enumerate() {
            for (j, b) in numbers.iter().enumerate().skip(i) {
                for c in numbers.iter().skip(j) {
                    if a + b + c == 2020 {
                        maybe_triplet = Some((*a, *b, *c));
                    }
                }
            }
        }
        maybe_triplet
    };

    match triplet {
        None => println!("Nothing sums to 2020"),
        Some(x) => {
            println!(
                "{0} + {1} + {2} == 2020, {0} * {1} * {2} = {3}",
                x.0,
                x.1,
                x.2,
                x.0 * x.1 * x.2
            );
        }
    }
}
