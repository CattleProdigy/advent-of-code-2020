use std::collections::BTreeMap;
use std::collections::HashMap;
fn last_turn_p1(v: &Vec<i64>, last: i64) -> i64 {
    let mut growing = v.to_vec();
    let starting = v.len() as i64;
    for i in 0..last {
        if i < starting {
            continue;
        }
        let last_number = growing.last().unwrap();
        let found = growing
            .iter()
            .enumerate()
            .rev()
            .skip(1)
            .find(|(_, x)| *x == last_number);
        let new_val = match found {
            Some((i, _)) => (growing.len() - 1 - i) as i64,
            None => 0,
        };

        growing.push(new_val);
    }
    *growing.last().unwrap()
}

fn last_turn_p2(v: &Vec<i64>, last: i64) -> i64 {
    let starting = v.len() as i64;
    let mut map = HashMap::<i64, i64>::new();
    for (i, x) in v.iter().take((starting - 1) as usize).enumerate() {
        map.insert(*x, i as i64);
        //   print!("{}, ", x);
    }
    let mut last_val = *v.last().unwrap();
    //print!("{}, ", last_val);
    for i in 0..last {
        println!("{}", i);
        if i < starting {
            continue;
        }
        // let last_number = growing.last().unwrap();

        let new_val = if map.contains_key(&last_val) {
            i - 1 - map[&last_val]
        } else {
            0
        };
        //   print!("{}, ", new_val);

        map.insert(last_val, i - 1);
        last_val = new_val;
    }
    //print!("\n");
    last_val
}

fn main() {
    println!("{}", last_turn_p2(&vec![0, 3, 6], 10));
    println!("{}", last_turn_p2(&vec![0, 3, 6], 2020));
    println!("{}", last_turn_p2(&vec![1, 3, 2], 2020));
    println!("{}", last_turn_p2(&vec![2, 1, 3], 2020));
    println!("{}", last_turn_p2(&vec![1, 2, 3], 2020));
    println!("{}", last_turn_p2(&vec![2, 3, 1], 2020));
    println!("{}", last_turn_p2(&vec![3, 2, 1], 2020));
    println!("{}", last_turn_p2(&vec![3, 1, 2], 2020));
    println!("{}", last_turn_p2(&vec![7, 14, 0, 17, 11, 1, 2], 2020));
    println!("{}", last_turn_p2(&vec![7, 14, 0, 17, 11, 1, 2], 30000000));
    // println!("{}", last_turn_p2(&vec![0, 3, 6], 30000000));
    // println!("{}", last_turn_p2(&vec![1, 3, 2], 30000000));
    // println!("{}", last_turn_p2(&vec![2, 1, 3], 30000000));
    // println!("{}", last_turn_p2(&vec![1, 2, 3], 30000000));
    // println!("{}", last_turn_p2(&vec![2, 3, 1], 30000000));
    // println!("{}", last_turn_p2(&vec![3, 2, 1], 30000000));
    // println!("{}", last_turn_p2(&vec![3, 1, 2], 30000000));
}
