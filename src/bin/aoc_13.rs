use std::collections::VecDeque;
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

fn find_p2(buses: &Vec<Option<i64>>) -> i64 {
    let buses_offsets: Vec<(i64, i64)> = buses
        .iter()
        .enumerate()
        .filter(|(_, b)| b.is_some())
        .map(|(i, b)| (i as i64, b.unwrap()))
        .collect();

    let first_id = buses_offsets.first().unwrap().1;


    // c0 * b0 = t0 
    // c1 * b1 != t0 + off1

    // c0 * b0 = t0 + err
    // c1 * b1 = t0 + off1 + err
    
    // c0 * b0 = t0 + x*b0
    // c1 * b1 = t0 + off1 + x*b0

    let offsets = buses_offsets.iter().map(|(i, _)| *i).collect::<Vec<_>>();
    println!("offsets: {:?}", offsets);

    let mut search_queue = VecDeque::<Vec<i64>>::new();
    search_queue.push_back(vec![0; buses_offsets.len()]);

    let mut result: Vec<i64> = vec![];
    while !search_queue.is_empty() {
        let coeffs = search_queue.pop_back().unwrap();
        println!("c: {:?}", coeffs);

        let timestamps = buses_offsets
            .iter()
            .zip(coeffs.iter())
            .map(|((_, b), c)| *b * c)
            .collect::<Vec<_>>();
        let first_ts = timestamps.first().unwrap();
        let target_timestamps = offsets.iter().map(|ofs| ofs + first_ts).collect::<Vec<_>>();

        if target_timestamps == timestamps {
            result = coeffs;
            break;
        }
        println!("ts: {:?}", timestamps);
        println!("tgt: {:?}", target_timestamps);

        let mut num_changed = 0;
        let mut new_coeffs = coeffs.to_vec();
        for (i, &ts) in timestamps.iter().enumerate() {
            let target = target_timestamps[i];
            let (_, bus_id) = buses_offsets[i];
            if ts < target {
                // calculate new value
                let diff = target - ts;
                let new_coeff = diff / bus_id;
                if new_coeff > 0 && new_coeff * bus_id <= diff {
                    new_coeffs[i] += new_coeff;
                    num_changed += 1;
                }
            }
        }
        if num_changed > 0 {
            search_queue.push_back(new_coeffs);
        } else if num_changed == 0 {
            let mut new_coeffs2 = coeffs.to_vec();
            new_coeffs2[0] += ;
            search_queue.push_back(new_coeffs2);
        }
        if timestamps[0] > 1068788 {
            panic!("fukc");
        }

        // if num_changed == 0 {
        //     let mut new_coeffs = coeffs.to_vec();
        //     new_coeffs[0] += 1;
        //     search_queue.push_back(new_coeffs);
        // }

        // if num_changed == 0 {
        //     let raw_diffs = timestamps
        //         .iter()
        //         .zip(target_timestamps.iter())
        //         .map(|(ts, tgt)| tgt - ts)
        //         .collect::<Vec<_>>();
        //     println!("diffs: {:?}", raw_diffs);
        //     let diffs = timestamps
        //         .iter()
        //         .zip(target_timestamps.iter())
        //         .map(|(ts, tgt)| tgt - ts)
        //         .filter(|x| *x > 0)
        //         .collect::<Vec<_>>();

        //     let mut new_coeffs = coeffs.to_vec();
        //     let first = diffs.first();
        //     if first.is_some() {
        //         let lcm = diffs.iter().fold(*first.unwrap(), |acc, &d| lcm(acc, d));
        //         new_coeffs[0] += lcm;
        //     } else {
        //         new_coeffs[0] += 1;
        //     }
        //     search_queue.push_back(new_coeffs);
        // }
    }

    result.first().unwrap() * buses_offsets.first().unwrap().1
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
}
