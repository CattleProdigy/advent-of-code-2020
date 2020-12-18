use std::collections::HashMap;
use std::env;
use std::fmt;
use std::fmt::Display;
use std::fs;

#[derive(Clone, Debug)]
struct Range<T> {
    min: T,
    max: T,
}

#[derive(Clone, Debug)]
struct Grid4D<ValType> {
    grid: HashMap<(i32, i32, i32, i32), ValType>,
    x_range: Range<i32>,
    y_range: Range<i32>,
    z_range: Range<i32>,
    w_range: Range<i32>,
    def: ValType,
}

impl<V: Display + Clone> Grid4D<V> {
    fn default(default: V) -> Grid4D<V> {
        let invert_range = Range::<i32> {
            min: i32::MAX,
            max: i32::MIN,
        };
        Grid4D::<V> {
            grid: HashMap::new(),
            x_range: invert_range.clone(),
            y_range: invert_range.clone(),
            z_range: invert_range.clone(),
            w_range: invert_range.clone(),
            def: default,
        }
    }
    fn get(&self, x: i32, y: i32, z: i32, w: i32) -> &V {
        self.grid.get(&(x, y, z, w)).unwrap_or(&self.def)
    }
    fn get_mut(&mut self, x: i32, y: i32, z: i32, w: i32) -> &mut V {
        self.x_range.min = x.min(self.x_range.min);
        self.x_range.max = x.max(self.x_range.max);
        self.y_range.min = y.min(self.y_range.min);
        self.y_range.max = y.max(self.y_range.max);
        self.z_range.min = z.min(self.z_range.min);
        self.z_range.max = z.max(self.z_range.max);
        self.w_range.min = w.min(self.w_range.min);
        self.w_range.max = w.max(self.w_range.max);
        self.grid.entry((x, y, z, w)).or_insert(self.def.clone())
    }

    fn print(&self) {
        for w in self.w_range.min..self.w_range.max + 1 {
            for z in self.z_range.min..self.z_range.max + 1 {
                println!("w={}, z={}\n", w, z);
                for y in self.y_range.min..self.y_range.max + 1 {
                    for x in self.x_range.min..self.x_range.max + 1 {
                        print!("{}", self.get(x, y, z, w));
                    }
                    print!("\n");
                }
            }
        }
        println!("\n");
    }
}

fn evolve_grid(grid: Grid4D<char>) -> Grid4D<char> {
    let mut g = grid;

    let mut new_active: Vec<(i32, i32, i32, i32)> = Vec::new();
    let mut new_inactive: Vec<(i32, i32, i32, i32)> = Vec::new();
    let mut neighbor_offsets = Vec::<(i32, i32, i32, i32)>::new();
    for &w in vec![-1, 0, 1].iter() {
        for &z in vec![-1, 0, 1].iter() {
            for &y in vec![-1, 0, 1].iter() {
                for &x in vec![-1, 0, 1].iter() {
                    if !(x == 0 && y == 0 && z == 0 && w == 0) {
                        neighbor_offsets.push((x, y, z, w));
                    }
                }
            }
        }
    }

    let mut new_neighbors: Vec<(i32, i32, i32, i32)> = Vec::new();

    for (xyz, &v) in g.grid.iter() {
        let neighbor_idxs = neighbor_offsets
            .iter()
            .map(|(x, y, z, w)| (x + xyz.0, y + xyz.1, z + xyz.2, w + xyz.3));
        let active_neighbors = neighbor_idxs
            .map(|(nx, ny, nz, nw)| {
                if !g.grid.contains_key(&(nx, ny, nz, nw)) {
                    new_neighbors.push((nx, ny, nz, nw));
                }
                *g.get(nx, ny, nz, nw)
            })
            .filter(|&n| n == '#')
            .count();
        if v == '#' {
            if !(active_neighbors == 2 || active_neighbors == 3) {
                new_inactive.push(*xyz);
            }
        } else if v == '.' {
            if active_neighbors == 3 {
                new_active.push(*xyz);
            }
        } else {
            panic!("unknown grid val");
        }
    }
    for xyz in new_neighbors.into_iter() {
        let neighbor_idxs = neighbor_offsets
            .iter()
            .map(|(x, y, z, w)| (x + xyz.0, y + xyz.1, z + xyz.2, w + xyz.3));
        let active_neighbors = neighbor_idxs
            .map(|(nx, ny, nz, nw)| *g.get(nx, ny, nz, nw))
            .filter(|&n| n == '#')
            .count();
        if active_neighbors == 3 {
            new_active.push(xyz);
        }
    }
    for (x, y, z, w) in new_active.iter() {
        *g.get_mut(*x, *y, *z, *w) = '#';
    }
    for (x, y, z, w) in new_inactive.iter() {
        *g.get_mut(*x, *y, *z, *w) = '.';
    }

    g
}

fn main() {
    // example
    {
        let mut grid = Grid4D::<char>::default('.');
        *grid.get_mut(-1, -1, 0, 0) = '.';
        *grid.get_mut(0, -1, 0, 0) = '#';
        *grid.get_mut(1, -1, 0, 0) = '.';
        *grid.get_mut(-1, 0, 0, 0) = '.';
        *grid.get_mut(0, 0, 0, 0) = '.';
        *grid.get_mut(1, 0, 0, 0) = '#';
        *grid.get_mut(-1, 1, 0, 0) = '#';
        *grid.get_mut(0, 1, 0, 0) = '#';
        *grid.get_mut(1, 1, 0, 0) = '#';
        grid.print();
        for _ in 0..6 {
            grid.print();
            grid = evolve_grid(grid);
        }
        grid.print();
        println!(
            "example: {}",
            grid.grid.values().filter(|&&x| x == '#').count()
        );
    }

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("We need exactly one arguemnt, got: {}", args.len() - 1);
    }

    let file_contents = fs::read_to_string(&args[1]).expect("couldn't read the file");
    let mut grid = Grid4D::<char>::default('.');
    let mut y: i32 = 0;
    for l in file_contents.lines() {
        let mut x: i32 = 0;
        for c in l.chars() {
            *grid.get_mut(x, y, 0, 0) = c;
            x += 1;
        }
        y += 1;
    }
    grid.print();
    for _ in 0..6 {
        grid = evolve_grid(grid);
    }
    println!("p2: {}", grid.grid.values().filter(|&&x| x == '#').count());
}
