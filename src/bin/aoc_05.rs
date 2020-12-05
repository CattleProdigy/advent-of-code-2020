use std::env;
use std::fs;

fn parse_line(line: &str) -> Option<(i32, i32, i32)> {
    if line.len() != 10 {
        None
    } else {
        let mut itr = line.chars();
        let row = {
            let mut row_low_bound: i32 = 0;
            let mut row_range: i32 = 64;
            for _i in 0..7 {
                match itr.next().unwrap() {
                    'F' => (),
                    'B' => row_low_bound += row_range,
                    _ => panic!("Unknown row specifier"),
                };
                row_range /= 2;
            }
            row_low_bound
        };
        let col = {
            let mut col_low_bound: i32 = 0;
            let mut col_range: i32 = 4;
            for _i in 0..3 {
                match itr.next().unwrap() {
                    'L' => (),
                    'R' => col_low_bound += col_range,
                    _ => panic!("Unknown col specifier"),
                };
                col_range /= 2;
            }
            col_low_bound
        };
        Some((row, col, row * 8 + col))
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("We need exactly one arguemnt, got: {}", args.len() - 1);
    }

    let file_contents = fs::read_to_string(&args[1]).expect("couldn't read the file");

    // Collect and sort by seat ID
    let seats = {
        let mut v = file_contents
            .split('\n')
            .filter(|x| !x.is_empty())
            .map(|x| parse_line(x).unwrap())
            .collect::<Vec<_>>();
        v.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
        v
    };

    {
        let max = seats.iter().last().unwrap().2;
        println!("part 1 max: {}", max);
    }

    {
        let mut prev = seats.iter();
        let mut next = seats.iter();

        // Skip the first element and move next ahead of prev
        // by one element
        prev.next();
        next.next();
        next.next();
        let result = loop {
            let prev_val = prev.next().unwrap().2;
            match next.next() {
                Some(next_val) => {
                    if prev_val + 1 == next_val.2 - 1 {
                        break Some(prev_val + 1);
                    }
                }
                None => break None,
            }
        };

        match result {
            Some(x) => println!("part 2 your seat: {}", x),
            None => println!("couldn't find a seat"),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        {
            let (r, c, id) = parse_line("FBFBBFFRLR").unwrap();
            assert_eq!(r, 44);
            assert_eq!(c, 5);
            assert_eq!(id, 357);
        }
        {
            let (r, c, id) = parse_line("BFFFBBFRRR").unwrap();
            assert_eq!(r, 70);
            assert_eq!(c, 7);
            assert_eq!(id, 567);
        }
        {
            let (r, c, id) = parse_line("FFFBBBFRRR").unwrap();
            assert_eq!(r, 14);
            assert_eq!(c, 7);
            assert_eq!(id, 119);
        }
        {
            let (r, c, id) = parse_line("BBFFBBFRLL").unwrap();
            assert_eq!(r, 102);
            assert_eq!(c, 4);
            assert_eq!(id, 820);
        }
    }
}
