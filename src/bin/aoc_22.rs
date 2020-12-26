use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::env;
use std::fs;
fn simulate(mut p1: VecDeque<i64>, mut p2: VecDeque<i64>) -> (usize, VecDeque<i64>, i64) {
    while !p1.is_empty() && !p2.is_empty() {
        let card_p1 = p1.pop_front().unwrap();
        let card_p2 = p2.pop_front().unwrap();

        match card_p1.partial_cmp(&card_p2).unwrap() {
            std::cmp::Ordering::Greater => {
                p1.push_back(card_p1);
                p1.push_back(card_p2);
            }
            std::cmp::Ordering::Less => {
                p2.push_back(card_p2);
                p2.push_back(card_p1);
            }
            std::cmp::Ordering::Equal => {
                panic!("impossible")
            }
        }
    }

    let (winner, winner_deck) = if p1.is_empty() { (2, p2) } else { (1, p1) };

    let sum = winner_deck
        .iter()
        .rev()
        .enumerate()
        .map(|(i, c)| (i + 1) as i64 * c)
        .sum::<i64>();
    (winner, winner_deck, sum)
}
fn simulate_p2(mut p1: VecDeque<i64>, mut p2: VecDeque<i64>) -> (bool, VecDeque<i64>) {
    let mut record: HashSet<(Vec<i64>, Vec<i64>)> = HashSet::new();

    while !p1.is_empty() && !p2.is_empty() {
        if !record.insert((p1.iter().copied().collect(), p2.iter().copied().collect())) {
            return (true, p1);
        }
        let card_p1 = p1.pop_front().unwrap();
        let card_p2 = p2.pop_front().unwrap();

        let p1_enough = card_p1 as usize <= p1.len();
        let p2_enough = card_p2 as usize <= p2.len();
        let p1_wins = if p1_enough && p2_enough {
            let sub_p1 = p1
                .iter()
                .cloned()
                .take(card_p1 as usize)
                .collect::<VecDeque<_>>();
            let sub_p2 = p2
                .iter()
                .cloned()
                .take(card_p2 as usize)
                .collect::<VecDeque<_>>();
            let sub_winner = simulate_p2_impl(sub_p1, sub_p2);
            sub_winner
        } else {
            card_p1 > card_p2
        };

        if p1_wins {
            p1.push_back(card_p1);
            p1.push_back(card_p2);
        } else {
            p2.push_back(card_p2);
            p2.push_back(card_p1);
        }
    }

    if p2.is_empty() {
        (true, p1)
    } else {
        (false, p2)
    }
}

fn simulate_p2_impl(mut p1: VecDeque<i64>, mut p2: VecDeque<i64>) -> bool {
    let mut record: HashSet<(Vec<i64>, Vec<i64>)> = HashSet::new();

    while !p1.is_empty() && !p2.is_empty() {
        if !record.insert((p1.iter().copied().collect(), p2.iter().copied().collect())) {
            return true;
        }
        let card_p1 = p1.pop_front().unwrap();
        let card_p2 = p2.pop_front().unwrap();

        let p1_enough = card_p1 as usize <= p1.len();
        let p2_enough = card_p2 as usize <= p2.len();
        let p1_wins = if p1_enough && p2_enough {
            let sub_p1 = p1
                .iter()
                .cloned()
                .take(card_p1 as usize)
                .collect::<VecDeque<_>>();
            let sub_p2 = p2
                .iter()
                .cloned()
                .take(card_p2 as usize)
                .collect::<VecDeque<_>>();
            let (sub_winner, _) = simulate_p2(sub_p1, sub_p2);
            sub_winner
        } else {
            card_p1 > card_p2
        };

        if p1_wins {
            p1.push_back(card_p1);
            p1.push_back(card_p2);
        } else {
            p2.push_back(card_p2);
            p2.push_back(card_p1);
        }
    }

    p2.is_empty()
}

fn main() {
    {
        let p1: VecDeque<i64> = VecDeque::from(vec![9, 2, 6, 3, 1]);
        let p2: VecDeque<i64> = VecDeque::from(vec![5, 8, 4, 7, 10]);
        {
            let (winner, deck, score) = simulate(p1.clone(), p2.clone());
            println!(
                "Winner is: {}, score: {}, with deck [{:?}]",
                winner, score, deck
            );
        }
        {
            let (winner, deck) = simulate_p2(p1, p2);
            let sum = deck
                .iter()
                .rev()
                .enumerate()
                .map(|(i, c)| (i + 1) as i64 * c)
                .sum::<i64>();
            println!(
                "Winner is: {}, score {}, with deck [{:?}]",
                if winner { 1 } else { 2 },
                sum,
                deck
            );
        }
    }

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("We need exactly one arguemnt, got: {}", args.len() - 1);
    }

    let file_contents = fs::read_to_string(&args[1]).expect("couldn't read the file");

    let mut map_iter = file_contents.split("\n\n").map(|p| {
        p.lines()
            .skip(1)
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<VecDeque<_>>()
    });
    let p1 = map_iter.next().unwrap();
    let p2 = map_iter.next().unwrap();

    {
        println!("p1 [{:?}]", p1);
        println!("p2 [{:?}]", p2);
        let (winner, deck, score) = simulate(p1.clone(), p2.clone());
        println!(
            "Winner is: {}, score: {}, with deck [{:?}]",
            winner, score, deck
        );
    }
    {
        let (winner, deck) = simulate_p2(p1, p2);
        let sum = deck
            .iter()
            .rev()
            .enumerate()
            .map(|(i, c)| (i + 1) as i64 * c)
            .sum::<i64>();
        println!(
            "Winner is: {}, score {}, with deck [{:?}]",
            if winner { 1 } else { 2 },
            sum,
            deck
        );
    }
}
