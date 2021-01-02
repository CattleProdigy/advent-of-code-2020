use std::env;
use std::fs;
fn gcd(a: i64, b: i64) -> i64 {
    match ((a, b), (a & 1, b & 1)) {
        ((x, y), _) if x == y => y,
        ((0, x), _) | ((x, 0), _) => x,
        ((x, y), (0, 1)) | ((y, x), (1, 0)) => gcd(x >> 1, y),
        ((x, y), (0, 0)) => gcd(x >> 1, y >> 1) << 1,
        ((x, y), (1, 1)) => {
            let (x, y) = (std::cmp::min(x, y), std::cmp::max(x, y));
            gcd((y - x) >> 1, x)
        }
        _ => unreachable!(),
    }
}

fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}

fn find_first_dep_p1(min_dep_time: i64, buses: &Vec<Option<i64>>) -> (i64, i64) {
    let mut dep = min_dep_time;
    loop {
        for b in buses.iter() {
            if b.is_some() {
                if dep % b.unwrap() == 0 {
                    return (dep, b.unwrap());
                }
            }
        }

        dep += 1;
    }
}

fn find_phase(a: i64, b: i64, offset: i64) -> i64 {
    println!("fp: {} {} {}", a, b, offset);
    let mut x = 0;
    loop {
        if (offset + x) % b == 0 {
            return offset + x;
        }

        x += a;
    }
}

fn find_p2(buses: &Vec<Option<i64>>) -> i64 {
    let buses_offsets: Vec<(i64, i64)> = buses
        .iter()
        .enumerate()
        .filter(|(_, b)| b.is_some())
        .map(|(i, b)| (i as i64, b.unwrap()))
        .collect();

    let offsets = buses_offsets.iter().map(|(i, _)| *i).collect::<Vec<_>>();
    println!("offsets: {:?}", offsets);

    let mut old_freq = buses_offsets.first().unwrap().1;
    let mut old_offset = 0;
    for i in 1..offsets.len() {
        let (offset, freq) = buses_offsets[i];
        println!("old: {} {}, new: {} {}", old_freq, old_offset, freq, offset);

        let new_phase = find_phase(old_freq, freq, offset + old_offset) - offset;
        let new_freq = lcm(old_freq, freq);

        println!("ph: {}, freq: {}", new_phase, new_freq);
        old_freq = new_freq;
        old_offset = new_phase;
    }

    old_offset
}

fn parse(input: &str) -> (i64, Vec<Option<i64>>) {
    let mut line_itr = input.lines();

    let departure_time = line_itr.next().unwrap().parse::<i64>().unwrap();

    let bus_ids = line_itr
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<i64>().ok())
        .collect::<Vec<_>>();

    (departure_time, bus_ids)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("We need exactly one arguemnt, got: {}", args.len() - 1);
    }

    let file_contents = fs::read_to_string(&args[1]).expect("couldn't read the file");

    let (departure, buses) = parse(&file_contents);
    let (dep, b) = find_first_dep_p1(departure, &buses);
    println!("p1: {}", b * (dep - departure));
    println!("p2: {}", find_p2(&buses));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let example1 = r#"939
7,13,x,x,59,x,31,19"#;

        let (departure, buses) = parse(&example1);
        assert_eq!(departure, 939);
        let truth: Vec<Option<i64>> = vec![
            Some(7),
            Some(13),
            None,
            None,
            Some(59),
            None,
            Some(31),
            Some(19),
        ];
        assert_eq!(buses, truth);

        let (dep, b) = find_first_dep_p1(departure, &buses);

        assert_eq!(dep, 944);
        assert_eq!(b, 59);

        let ts = find_p2(&buses);
        assert_eq!(ts, 1068781);
    }

    #[test]
    fn test2() {
        let example1 = r#"939
1789,37,47,1889"#;

        let (departure, buses) = parse(&example1);
        let ts = find_p2(&buses);
        assert_eq!(ts, 1202161486);
    }
}
