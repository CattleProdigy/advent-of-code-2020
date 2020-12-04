use std::env;
use std::fs;

fn parse(file: &str) -> (String, usize, usize) {
    let h = file.split('\n').count() - 1;
    let flat_str = file
        .split('\n')
        .filter(|x| !x.is_empty())
        .collect::<String>();
    let w = flat_str.len() / h;
    (flat_str, w, h)
}

fn traverse(flat_str: &str, width: usize, height: usize, slope_x: usize, slope_y: usize) -> usize {
    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut count: usize = 0;

    while y < height {
        let x_mod = x % width;
        let index = x_mod + y * width;
        let value = flat_str.chars().nth(index).unwrap();
        if value == '#' {
            count = count + 1;
        }
        x += slope_x;
        y += slope_y;
    }

    count
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("We need exactly one arguemnt, got: {}", args.len() - 1);
    }

    let file_contents = fs::read_to_string(&args[1]).expect("couldn't read the file");

    let (flat_str, w, h) = parse(&file_contents);
    println!("w {} h {}", w, h);

    let tree_count_p1 = traverse(&flat_str, w, h, 3, 1);
    println!("part 1: tree count: {}", tree_count_p1);

    let product = [(1usize, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|(sx, sy)| traverse(&flat_str, w, h, *sx, *sy))
        .product::<usize>();

    println!("part 2: {}", product);
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = r#"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#
"#;

    #[test]
    fn test_parse() {
        let (_flat_str, w, h) = parse(EXAMPLE);
        assert_eq!(w, 11);
        assert_eq!(h, 11);
    }

    #[test]
    fn test_traverse() {
        let (flat_str, w, h) = parse(EXAMPLE);
        let tree_count = traverse(&flat_str, w, h, 3, 1);
        assert_eq!(tree_count, 7);
    }

    #[test]
    fn test_part2() {
        let (flat_str, w, h) = parse(EXAMPLE);
        let tree_count = traverse(&flat_str, w, h, 3, 1);
        let product = [(1usize, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
            .iter()
            .map(|(sx, sy)| traverse(&flat_str, w, h, *sx, *sy))
            .product::<usize>();
        assert_eq!(product, 336);
    }
}
