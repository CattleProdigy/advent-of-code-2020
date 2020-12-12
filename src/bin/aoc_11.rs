use std::env;
use std::fs;

#[derive(Clone, Debug)]
struct Grid2D {
    grid: Vec<char>,
    w: i32,
    h: i32,
}

fn parse(grid_str: &str) -> Grid2D {
    let h = grid_str.lines().count() as i32;
    let grid = grid_str
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .flatten()
        .collect::<Vec<_>>();
    let w = grid.len() as i32 / h;

    Grid2D {
        grid: grid,
        w: w,
        h: h,
    }
}

fn _print(grid: &Grid2D) {
    let g = &grid.grid;
    let w = grid.w;
    let h = grid.h;
    for y in 0..h {
        for x in 0..w {
            let idx = y * w + x;
            print!("{}", g[idx as usize]);
        }
        print!("\n");
    }
}

fn evolve_p1(grid: &mut Grid2D) -> usize {
    let mut changed: usize = 0;
    let mut grid_copy = grid.clone();

    let g = &grid.grid;
    let w = grid.w;
    let h = grid.h;
    let neighbor_offsets = vec![-1, 0, 1]
        .iter()
        .map(|x| {
            vec![-1, 0, 1]
                .into_iter()
                .map(|y| (*x, y))
                .collect::<Vec<(i32, i32)>>()
        })
        .flatten()
        .filter(|(x, y)| !(*x == 0 && *y == 0))
        .collect::<Vec<_>>();
    for y in 0..h {
        for x in 0..w {
            let idx = (y * w + x) as usize;
            let cur = g[idx];
            if cur == '.' {
                continue;
            }

            let occupied_neighbors = neighbor_offsets
                .iter()
                .map(|(xo, yo)| (xo + x, yo + y))
                .filter(|(nx, ny)| *nx >= 0 && *nx < w && *ny >= 0 && *ny < h)
                .map(|(nx, ny)| {
                    let nidx = (ny * w + nx) as usize;
                    g[nidx]
                })
                .filter(|&n| n == '#')
                .count();

            if cur == '#' && occupied_neighbors >= 4 {
                grid_copy.grid[idx] = 'L';
                changed += 1;
            } else if cur == 'L' && occupied_neighbors == 0 {
                grid_copy.grid[idx] = '#';
                changed += 1;
            }
        }
    }
    *grid = grid_copy;

    changed
}

fn evolve_p2(grid: &mut Grid2D) -> usize {
    let mut changed: usize = 0;
    let mut grid_copy = grid.clone();

    let g = &grid.grid;
    let w = grid.w;
    let h = grid.h;

    let in_range = |x: i32, y: i32| x >= 0 && x < w && y >= 0 && y < h;

    let neighbor_dirs = vec![-1, 0, 1]
        .iter()
        .map(|x| {
            vec![-1, 0, 1]
                .into_iter()
                .map(|y| (*x, y))
                .collect::<Vec<(i32, i32)>>()
        })
        .flatten()
        .filter(|(x, y)| !(*x == 0 && *y == 0))
        .collect::<Vec<_>>();
    for y in 0..h {
        for x in 0..w {
            let idx = (y * w + x) as usize;
            let cur = g[idx];
            if cur == '.' {
                continue;
            }

            let occupied_neighbors = neighbor_dirs
                .iter()
                .map(|(xd, yd)| {
                    let mut cur_x = x;
                    let mut cur_y = y;
                    loop {
                        cur_x += xd;
                        cur_y += yd;

                        if !in_range(cur_x, cur_y) {
                            return '.';
                        }
                        let nidx = (cur_y * w + cur_x) as usize;
                        match g[nidx] {
                            'L' => return 'L',
                            '#' => return '#',
                            _ => (),
                        };
                    }
                })
                .filter(|&n| n == '#')
                .count();

            if cur == '#' && occupied_neighbors >= 5 {
                grid_copy.grid[idx] = 'L';
                changed += 1;
            } else if cur == 'L' && occupied_neighbors == 0 {
                grid_copy.grid[idx] = '#';
                changed += 1;
            }
        }
    }
    *grid = grid_copy;

    changed
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("We need exactly one arguemnt, got: {}", args.len() - 1);
    }

    let file_contents = fs::read_to_string(&args[1]).expect("couldn't read the file");

    {
        let mut grid = parse(&file_contents);
        loop {
            let changed = evolve_p1(&mut grid);
            if changed == 0 {
                break;
            }
        }

        let num_occ = grid.grid.iter().filter(|&x| *x == '#').count();
        println!("part1: {}", num_occ);
    }

    {
        let mut grid = parse(&file_contents);
        loop {
            let changed = evolve_p2(&mut grid);
            if changed == 0 {
                break;
            }
        }

        let num_occ = grid.grid.iter().filter(|&x| *x == '#').count();
        println!("part2: {}", num_occ);
    }
}
