use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::str::FromStr;

#[derive(Debug)]
enum Step {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

impl FromStr for Step {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "e" => Ok(Step::East),
            "se" => Ok(Step::SouthEast),
            "sw" => Ok(Step::SouthWest),
            "w" => Ok(Step::West),
            "ne" => Ok(Step::NorthEast),
            "nw" => Ok(Step::NorthWest),
            _ => Err(format!("Unknown step string: {}", s)),
        }
    }
}

fn steps_to_coor(steps: &Vec<Step>) -> (i64, i64) {
    steps.iter().fold((0, 0), |acc, s| {
        let s_coor = match s {
            Step::East => (2, 0),
            Step::SouthEast => (1, -2),
            Step::SouthWest => (-1, -2),
            Step::West => (-2, 0),
            Step::NorthWest => (-1, 2),
            Step::NorthEast => (1, 2),
        };
        (acc.0 + s_coor.0, acc.1 + s_coor.1)
    })
}

fn parse(input: &str) -> Vec<Vec<Step>> {
    let re = Regex::new("((?:se|sw|ne|nw|e|w|))").unwrap();

    input
        .lines()
        .map(|l| {
            let caps = re.captures_iter(l);
            caps.map(|c| Step::from_str(&c[1]).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn evolve(map: &mut HashMap<(i64, i64), bool>) {
    let mut new_black: Vec<(i64, i64)> = Vec::new();
    let mut new_white: Vec<(i64, i64)> = Vec::new();
    let mut new_neighbors: Vec<(i64, i64)> = Vec::new();

    let neighbors = vec![(2, 0), (1, -2), (-1, -2), (-2, 0), (-1, 2), (1, 2)];

    for ((x, y), v) in map.iter() {
        let mut black_count = 0;
        for (offx, offy) in neighbors.iter() {
            let neigh_xy = (x + offx, y + offy);
            let map_neigh = map.get(&neigh_xy);
            if map_neigh.is_none() {
                new_neighbors.push(neigh_xy);
            }
            let neigh_val = *map_neigh.unwrap_or(&false);
            if neigh_val {
                black_count += 1
            }
        }
        if *v {
            // black
            if black_count == 0 || black_count > 2 {
                new_white.push((*x, *y));
            }
        } else {
            // white
            if black_count == 2 {
                new_black.push((*x, *y));
            }
        }
    }
    for (x, y) in new_neighbors.iter() {
        let mut black_count = 0;
        for (offx, offy) in neighbors.iter() {
            let neigh_xy = (x + offx, y + offy);
            let map_neigh = map.get(&neigh_xy);
            let neigh_val = *map_neigh.unwrap_or(&false);
            if neigh_val {
                black_count += 1
            }
        }
        // white
        if black_count == 2 {
            new_black.push((*x, *y));
        }
    }

    for xy in new_black {
        map.entry(xy).and_modify(|x| *x = true).or_insert(true);
    }
    for xy in new_white {
        map.entry(xy).and_modify(|x| *x = false).or_insert(false);
    }
}

fn main() {
    let _example = r#"sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew"#;

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("We need exactly one arguemnt, got: {}", args.len() - 1);
    }

    let file_contents = fs::read_to_string(&args[1]).expect("couldn't read the file");

    let steps = parse(&file_contents);

    {
        let mut map: HashMap<(i64, i64), bool> = HashMap::new();
        for s in steps.iter() {
            let coor = steps_to_coor(&s);
            map.entry(coor).and_modify(|x| *x = !(*x)).or_insert(true);
        }

        println!("P1: {}", map.values().filter(|&x| *x).count());

        for _ in 1..101 {
            evolve(&mut map);
        }
        println!("P2: {}", map.values().filter(|&x| *x).count());
    }
}
