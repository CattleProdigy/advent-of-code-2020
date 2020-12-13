use std::env;
use std::fs;

#[derive(Debug, Clone)]
struct ShipState {
    heading: i32,
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct ShipWaypointState {
    ship_x: i32,
    ship_y: i32,
    waypoint_x: i32,
    waypoint_y: i32,
}

fn ship_step_p1(ship: ShipState, command: char, val: i32) -> ShipState {
    let mut new_state = ship.clone();
    match command {
        'N' => new_state.y -= val,
        'S' => new_state.y += val,
        'E' => new_state.x += val,
        'W' => new_state.x -= val,
        'L' => new_state.heading += val,
        'R' => new_state.heading -= val,
        'F' => match new_state.heading {
            0 => new_state.x += val,
            90 => new_state.y -= val,
            180 => new_state.x -= val,
            270 => new_state.y += val,
            _ => panic!("shouldn't happen"),
        },
        _ => panic!("Unknown Command!"),
    };

    if new_state.heading >= 360 {
        new_state.heading = new_state.heading % 360;
    } else if new_state.heading < 0 {
        let flipped = (-new_state.heading) % 360;
        new_state.heading = 360 - flipped;
    }

    new_state
}

fn step_p2(state: ShipWaypointState, command: char, val: i32) -> ShipWaypointState {
    let mut new_state = state.clone();
    match command {
        'N' => new_state.waypoint_y -= val,
        'S' => new_state.waypoint_y += val,
        'E' => new_state.waypoint_x += val,
        'W' => new_state.waypoint_x -= val,
        'R' => match val {
            90 => {
                new_state.waypoint_x = -state.waypoint_y;
                new_state.waypoint_y = state.waypoint_x;
            }
            180 => {
                new_state.waypoint_x = -state.waypoint_x;
                new_state.waypoint_y = -state.waypoint_y;
            }
            270 => {
                new_state.waypoint_x = state.waypoint_y;
                new_state.waypoint_y = -state.waypoint_x;
            }
            _ => panic!("unknown rotation"),
        },
        'L' => match val {
            90 => {
                new_state.waypoint_x = state.waypoint_y;
                new_state.waypoint_y = -state.waypoint_x;
            }
            180 => {
                new_state.waypoint_x = -state.waypoint_x;
                new_state.waypoint_y = -state.waypoint_y;
            }
            270 => {
                new_state.waypoint_x = -state.waypoint_y;
                new_state.waypoint_y = state.waypoint_x;
            }
            _ => panic!("unknown rotation"),
        },
        'F' => {
            new_state.ship_x += new_state.waypoint_x * val;
            new_state.ship_y += new_state.waypoint_y * val;
        }
        _ => panic!("Unknown Command!"),
    };

    new_state
}

fn parse(input_str: &str) -> Vec<(char, i32)> {
    input_str
        .lines()
        .map(|x| {
            let char_vec: Vec<char> = x.chars().collect();
            let command = char_vec[0];
            let val = char_vec[1..]
                .iter()
                .collect::<String>()
                .parse::<i32>()
                .unwrap();
            (command, val)
        })
        .collect::<Vec<(char, i32)>>()
}

fn main() {
    {
        let example = r#"F10
N3
F7
R90
F11"#;

        let commands = parse(&example);

        println!("{:?}", commands);
        {
            let mut ss = ShipState {
                heading: 0,
                x: 0,
                y: 0,
            };
            println!("{:?}", ss);
            for command in commands.iter() {
                ss = ship_step_p1(ss, command.0, command.1);
                println!("{:?}", ss);
            }
        }
        {
            let mut sws = ShipWaypointState {
                ship_x: 0,
                ship_y: 0,
                waypoint_x: 10,
                waypoint_y: -1,
            };
            println!("{:?}", sws);
            for command in commands.iter() {
                sws = step_p2(sws, command.0, command.1);
                println!("{:?}", sws);
            }
        }
    }

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("We need exactly one arguemnt, got: {}", args.len() - 1);
    }

    let file_contents = fs::read_to_string(&args[1]).expect("couldn't read the file");
    let mut ss = ShipState {
        heading: 0,
        x: 0,
        y: 0,
    };
    let commands = parse(&file_contents);
    for command in commands.iter() {
        ss = ship_step_p1(ss, command.0, command.1);
    }
    println!("p1: {}", ss.x.abs() + ss.y.abs());

    let mut sws = ShipWaypointState {
        ship_x: 0,
        ship_y: 0,
        waypoint_x: 10,
        waypoint_y: -1,
    };
    for command in commands.iter() {
        sws = step_p2(sws, command.0, command.1);
    }
    println!("p2: {}", sws.ship_x.abs() + sws.ship_y.abs());
}
