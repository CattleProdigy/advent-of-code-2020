use std::collections::HashMap;
use std::collections::VecDeque;
use std::convert::TryInto;
use std::env;
use std::fs;

const TILE_WH: usize = 10;

#[derive(Debug, Clone)]
struct Tile {
    id: usize,
    top_edge: [char; TILE_WH],
    bottom_edge: [char; TILE_WH],
    left_edge: [char; TILE_WH],
    right_edge: [char; TILE_WH],
    tile_contents: [char; TILE_WH * TILE_WH],
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Orientation {
    cw_turns: usize,
    flip_lr: bool,
    flip_tb: bool,
}

fn up_aligning_orientations(edges: [bool; 8]) -> Vec<Orientation> {
    let mut orientations: Vec<Orientation> = Vec::new();
    if edges[0] {
        orientations.push(Orientation {
            cw_turns: 0,
            flip_lr: false,
            flip_tb: false,
        });
    }
    if edges[1] {
        orientations.push(Orientation {
            cw_turns: 0,
            flip_lr: true,
            flip_tb: false,
        });
    }
    if edges[2] {
        orientations.push(Orientation {
            cw_turns: 2,
            flip_lr: false,
            flip_tb: false,
        });
    }
    if edges[3] {
        orientations.push(Orientation {
            cw_turns: 2,
            flip_lr: true,
            flip_tb: false,
        });
    }
    if edges[4] {
        orientations.push(Orientation {
            cw_turns: 1,
            flip_lr: false,
            flip_tb: false,
        });
    }
    if edges[5] {
        orientations.push(Orientation {
            cw_turns: 1,
            flip_lr: true,
            flip_tb: false,
        });
    }
    if edges[6] {
        orientations.push(Orientation {
            cw_turns: 3,
            flip_lr: false,
            flip_tb: false,
        });
    }
    if edges[7] {
        orientations.push(Orientation {
            cw_turns: 3,
            flip_lr: true,
            flip_tb: false,
        });
    }

    orientations
}

fn left_aligning_orientations(edges: [bool; 8]) -> Vec<Orientation> {
    let mut orientations: Vec<Orientation> = Vec::new();
    if edges[0] {
        orientations.push(Orientation {
            cw_turns: 3,
            flip_lr: false,
            flip_tb: false,
        });
    }
    if edges[1] {
        orientations.push(Orientation {
            cw_turns: 3,
            flip_lr: false,
            flip_tb: true,
        });
    }
    if edges[2] {
        orientations.push(Orientation {
            cw_turns: 1,
            flip_lr: false,
            flip_tb: false,
        });
    }
    if edges[3] {
        orientations.push(Orientation {
            cw_turns: 1,
            flip_lr: false,
            flip_tb: true,
        });
    }
    if edges[4] {
        orientations.push(Orientation {
            cw_turns: 0,
            flip_lr: false,
            flip_tb: false,
        });
    }
    if edges[5] {
        orientations.push(Orientation {
            cw_turns: 0,
            flip_lr: false,
            flip_tb: true,
        });
    }
    if edges[6] {
        orientations.push(Orientation {
            cw_turns: 2,
            flip_lr: false,
            flip_tb: false,
        });
    }
    if edges[7] {
        orientations.push(Orientation {
            cw_turns: 2,
            flip_lr: false,
            flip_tb: true,
        });
    }

    orientations
}

impl Tile {
    fn has_edge(&self, edge: &[char; TILE_WH]) -> [bool; 8] {
        [
            edge.iter().eq(self.top_edge.iter()),
            edge.iter().eq(self.top_edge.iter().rev()),
            edge.iter().eq(self.bottom_edge.iter()),
            edge.iter().eq(self.bottom_edge.iter().rev()),
            edge.iter().eq(self.left_edge.iter()),
            edge.iter().eq(self.left_edge.iter().rev()),
            edge.iter().eq(self.right_edge.iter()),
            edge.iter().eq(self.right_edge.iter().rev()),
        ]
    }

    fn rotate(&self) -> Tile {
        let mut rotated_contents = self.tile_contents;
        let mut source_idx: usize = 0;
        for x in (0..TILE_WH).rev() {
            for y in 0..TILE_WH {
                let dest_idx = y * TILE_WH + x;
                rotated_contents[dest_idx] = self.tile_contents[source_idx];
                source_idx += 1;
            }
        }

        Tile {
            id: self.id,
            top_edge: self.left_edge,
            bottom_edge: self.right_edge,
            left_edge: self.bottom_edge,
            right_edge: self.top_edge,
            tile_contents: rotated_contents,
        }
    }

    fn flip_lr(&self) -> Tile {
        let mut flipped_contents = self.tile_contents;
        let mut source_idx: usize = 0;
        for y in 0..TILE_WH {
            for x in (0..TILE_WH).rev() {
                let dest_idx = y * TILE_WH + x;
                flipped_contents[dest_idx] = self.tile_contents[source_idx];
                source_idx += 1;
            }
        }
        let mut t_tmp = self.top_edge;
        t_tmp.reverse();
        let mut b_tmp = self.bottom_edge;
        b_tmp.reverse();
        let mut l_tmp = self.left_edge;
        l_tmp.reverse();
        let mut r_tmp = self.right_edge;
        r_tmp.reverse();
        Tile {
            id: self.id,
            top_edge: t_tmp,
            bottom_edge: b_tmp,
            left_edge: r_tmp,
            right_edge: l_tmp,
            tile_contents: flipped_contents,
        }
    }
    fn flip_tb(&self) -> Tile {
        let mut flipped_contents = self.tile_contents;
        let mut source_idx: usize = 0;
        for y in (0..TILE_WH).rev() {
            for x in 0..TILE_WH {
                let dest_idx = y * TILE_WH + x;
                flipped_contents[dest_idx] = self.tile_contents[source_idx];
                source_idx += 1;
            }
        }
        let mut t_tmp = self.top_edge;
        t_tmp.reverse();
        let mut b_tmp = self.bottom_edge;
        b_tmp.reverse();
        let mut l_tmp = self.left_edge;
        l_tmp.reverse();
        let mut r_tmp = self.right_edge;
        r_tmp.reverse();
        Tile {
            id: self.id,
            top_edge: b_tmp,
            bottom_edge: t_tmp,
            left_edge: l_tmp,
            right_edge: r_tmp,
            tile_contents: flipped_contents,
        }
    }

    fn _print(&self) -> String {
        let mut s: String = "".to_string();
        for chk in self.tile_contents.chunks(TILE_WH) {
            for c in chk.iter() {
                s.push(*c);
            }
            s.push('\n');
        }

        s
    }
}

fn parse(input: &str) -> Vec<Tile> {
    input
        .split("\n\n")
        .map(|chunk| {
            let mut l_iter = chunk.lines();
            let tile_line = l_iter.next().unwrap();
            let space_pos = tile_line.find(' ').unwrap();
            let colon_pos = tile_line.find(':').unwrap();
            let tile = tile_line[space_pos + 1..colon_pos]
                .parse::<usize>()
                .unwrap();

            let tile_contents = l_iter
                .map(|l| l.chars().collect::<Vec<_>>())
                .flatten()
                .collect::<Vec<_>>();

            assert_eq!(tile_contents.len(), TILE_WH * TILE_WH);
            let top_edge = &tile_contents[0..TILE_WH];
            let bottom_edge = &tile_contents[(TILE_WH - 1) * TILE_WH..];
            let left_edge = tile_contents
                .iter()
                .cloned()
                .step_by(TILE_WH)
                .collect::<Vec<_>>();
            let right_edge = tile_contents
                .iter()
                .cloned()
                .skip(TILE_WH - 1)
                .step_by(TILE_WH)
                .collect::<Vec<_>>();

            // Storage is clockwise in the canonical frame
            let mut t = Tile {
                id: tile,
                top_edge: top_edge.try_into().unwrap(),
                bottom_edge: bottom_edge.try_into().unwrap(),
                left_edge: left_edge.as_slice().try_into().unwrap(),
                right_edge: right_edge.try_into().unwrap(),
                tile_contents: tile_contents.try_into().unwrap(),
            };
            t.bottom_edge.reverse();
            t.left_edge.reverse();

            t
        })
        .collect::<Vec<_>>()
}

fn align_grid(tiles: &Vec<Tile>) {
    let grid_dim = (tiles.len() as f32).sqrt() as usize;
    assert_eq!(grid_dim * grid_dim, tiles.len());

    let tiles_map = tiles
        .iter()
        .map(|t| (t.id, (t.clone())))
        .collect::<HashMap<_, _>>();

    let mut corner_ids: Vec<usize> = Vec::new();
    let mut edge_ids: Vec<usize> = Vec::new();
    let mut middle_ids: Vec<usize> = Vec::new();
    for i in tiles.iter() {
        let mut match_count = 0;
        let edges = [i.top_edge, i.bottom_edge, i.left_edge, i.right_edge];
        for j in tiles.iter() {
            if i.id == j.id {
                continue;
            }
            for e in edges.iter() {
                if j.has_edge(e).iter().any(|x| *x) {
                    match_count += 1;
                    break;
                }
            }
        }
        match match_count {
            2 => corner_ids.push(i.id),
            3 => edge_ids.push(i.id),
            4 => middle_ids.push(i.id),
            _ => {
                panic!("a tile should match 2,3,4 others");
            }
        }
    }

    assert_eq!(corner_ids.len(), 4);
    assert_eq!(edge_ids.len(), 4 * (grid_dim - 2));
    assert_eq!(
        middle_ids.len(),
        tiles.len() - (corner_ids.len() + edge_ids.len())
    );

    println!(
        "p1: corner id product: {}",
        corner_ids.iter().product::<usize>()
    );

    let next_pt = |(x, y): (usize, usize)| {
        if y == grid_dim - 1 && x == grid_dim - 1 {
            None
        } else if x == grid_dim - 1 {
            Some((0, y + 1))
        } else {
            Some((x + 1, y))
        }
    };
    let is_corner_pt = |xy: (usize, usize)| {
        let max = grid_dim - 1;
        match xy {
            (0, 0) => true,
            k if (k == (0, max)) => true,
            k if (k == (max, 0)) => true,
            k if (k == (max, max)) => true,
            _ => false,
        }
    };
    let is_edge_pt = |xy: (usize, usize)| {
        if is_corner_pt(xy) {
            false
        } else {
            let max = grid_dim - 1;
            match xy {
                (0, _) => true,
                (_, 0) => true,
                k if (k.0 == max) => true,
                k if (k.1 == max) => true,
                _ => false,
            }
        }
    };

    let mut assignments: HashMap<(usize, usize), Option<Tile>> = HashMap::new();
    for y in 0..grid_dim {
        for x in 0..grid_dim {
            assignments.insert((x, y), None);
        }
    }
    let mut id_assignments: HashMap<usize, Option<(usize, usize)>> = HashMap::new();
    for i in tiles.iter() {
        id_assignments.insert(i.id, None);
    }

    let mut search_queue: VecDeque<((usize, usize), Tile)> = VecDeque::new();
    for ci in corner_ids.iter() {
        let mut unmatched_edges: [bool; 4] = [false; 4];

        let ci_tile = tiles_map[ci].clone();
        let edges = [
            ci_tile.top_edge,
            ci_tile.bottom_edge,
            ci_tile.left_edge,
            ci_tile.right_edge,
        ];
        for (i, e) in edges.iter().enumerate() {
            for j in tiles.iter() {
                if *ci == j.id {
                    continue;
                }
                let matched = j.has_edge(e).iter().any(|x| *x);
                if matched {
                    unmatched_edges[i] = matched;
                    break;
                }
            }
        }

        for m in unmatched_edges.iter_mut() {
            *m = !(*m);
        }
        let mut orientations: Vec<Orientation> = Vec::new();
        // 0top 1 bottom 2 left 3 right
        if unmatched_edges[0] && unmatched_edges[2] {
            // top and left
            orientations.push(Orientation {
                cw_turns: 0,
                flip_lr: false,
                flip_tb: false,
            });
            orientations.push(Orientation {
                cw_turns: 3,
                flip_lr: false,
                flip_tb: true,
            });
        } else if unmatched_edges[0] && unmatched_edges[3] {
            // top and right
            orientations.push(Orientation {
                cw_turns: 0,
                flip_lr: true,
                flip_tb: false,
            });
            orientations.push(Orientation {
                cw_turns: 3,
                flip_lr: false,
                flip_tb: false,
            });
        } else if unmatched_edges[1] && unmatched_edges[2] {
            //   bottom and left
            orientations.push(Orientation {
                cw_turns: 1,
                flip_lr: false,
                flip_tb: false,
            });
            orientations.push(Orientation {
                cw_turns: 2,
                flip_lr: true,
                flip_tb: false,
            });
        } else if unmatched_edges[1] && unmatched_edges[3] {
            //   bottom and right
            orientations.push(Orientation {
                cw_turns: 2,
                flip_lr: false,
                flip_tb: false,
            });
            orientations.push(Orientation {
                cw_turns: 1,
                flip_lr: true,
                flip_tb: true,
            });
        } else {
            panic!("aaa!");
        }
        for o in orientations.iter() {
            let mut tile = tiles_map[ci].clone();
            for _ in 0..o.cw_turns {
                tile = tile.rotate();
            }
            if o.flip_lr {
                tile = tile.flip_lr();
            }
            if o.flip_tb {
                tile = tile.flip_tb();
            }
            search_queue.push_front(((0, 0), tile));
        }
    }

    while !search_queue.is_empty() {
        let (xy_cur, tile) = search_queue.pop_back().unwrap();

        // clear everything past us in case we popped from far ahead
        let mut xy_clear = if xy_cur == (0, 0) { None } else { Some(xy_cur) };
        while let Some(xy) = xy_clear {
            let maybe_tile = assignments[&xy].as_ref();
            if maybe_tile.is_none() {
                break;
            }
            let tile = assignments[&xy].as_ref().unwrap();
            *id_assignments.get_mut(&tile.id).unwrap() = None;
            *assignments.get_mut(&xy).unwrap() = None;
            xy_clear = next_pt(xy);
        }

        *id_assignments.get_mut(&(tile.id.clone())).unwrap() = Some(xy_cur);
        *assignments.get_mut(&xy_cur).unwrap() = Some(tile);

        let maybe_xy_next = next_pt(xy_cur);

        if maybe_xy_next.is_none() {
            println!("Done!");
            break;
        }
        let xy_next = maybe_xy_next.unwrap();

        let available_tiles: Vec<usize> = if is_corner_pt(xy_next) {
            corner_ids
                .iter()
                .copied()
                .filter(|c| id_assignments[c].is_none())
                .collect::<Vec<_>>()
        } else if is_edge_pt(xy_next) {
            edge_ids
                .iter()
                .copied()
                .filter(|c| id_assignments[c].is_none())
                .collect::<Vec<_>>()
        } else {
            middle_ids
                .iter()
                .copied()
                .filter(|c| id_assignments[c].is_none())
                .collect::<Vec<_>>()
        };

        let up_constraint = {
            if xy_next.1 == 0 {
                None
            } else {
                let up_tile = assignments[&(xy_next.0, xy_next.1 - 1)].as_ref().unwrap();
                let mut be = up_tile.bottom_edge;
                be.reverse();
                Some(be)
            }
        };
        let left_constraint = {
            if xy_next.0 == 0 {
                None
            } else {
                let left_tile = assignments[&(xy_next.0 - 1, xy_next.1)].as_ref().unwrap();
                let mut re = left_tile.right_edge;
                re.reverse();
                Some(re)
            }
        };

        for at in available_tiles.iter() {
            let mut ua_or: Vec<Orientation> = Vec::new();
            if let Some(uc) = up_constraint {
                let edge = tiles_map[at].has_edge(&uc);
                if edge.iter().any(|x| *x) {
                    ua_or = up_aligning_orientations(edge);
                } else {
                    // constraint unfulfilled
                    continue;
                }
            }
            let mut la_or: Vec<Orientation> = Vec::new();
            if let Some(lc) = left_constraint {
                let edge = tiles_map[at].has_edge(&lc);
                if edge.iter().any(|x| *x) {
                    la_or = left_aligning_orientations(edge);
                } else {
                    // constraint unfulfilled
                    continue;
                }
            }

            for o in ua_or.iter().chain(la_or.iter()) {
                let mut tile = tiles_map[at].clone();
                for _ in 0..o.cw_turns {
                    tile = tile.rotate();
                }
                if o.flip_lr {
                    tile = tile.flip_lr();
                }
                if o.flip_tb {
                    tile = tile.flip_tb();
                }
                let mut matches_up = true;
                let mut matches_left = true;
                if let Some(uc) = up_constraint {
                    matches_up = tile.top_edge.iter().eq(uc.iter());
                }
                if let Some(lc) = left_constraint {
                    matches_left = tile.left_edge.iter().eq(lc.iter());
                }

                if matches_up && matches_left {
                    search_queue.push_back((xy_next, tile));
                }
            }
        }
    }

    let inner_dim = TILE_WH - 2;
    let packed_dim = inner_dim * grid_dim;

    let mut final_grid: Vec<char> = vec!['.'; packed_dim * packed_dim];

    for gy in 0..grid_dim {
        for gx in 0..grid_dim {
            let tile = assignments[&(gx, gy)].as_ref().unwrap();
            let starting_addr = gy * packed_dim * inner_dim + gx * inner_dim;

            for ty in 0..inner_dim {
                for tx in 0..inner_dim {
                    let idx = (ty + 1) * TILE_WH + (tx + 1);
                    let tile_value = tile.tile_contents[idx];
                    final_grid[starting_addr + tx + ty * packed_dim] = tile_value;
                }
            }
        }
    }

    let mut s: String = "".to_string();
    for chk in final_grid.chunks(packed_dim) {
        for c in chk.iter() {
            s.push(*c);
        }
        s.push('\n');
    }

    let pattern = r#"                  # 
#    ##    ##    ###
 #  #  #  #  #  #   "#;
    let pattern_offsets = pattern
        .lines()
        .enumerate()
        .map(|(i, l)| {
            l.char_indices()
                .filter(|(_, c)| *c == '#')
                .map(|(j, _)| (j, i))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<_>>();
    let pattern_w = pattern_offsets.iter().map(|xy| xy.0).max().unwrap();
    let pattern_h = pattern_offsets.iter().map(|xy| xy.1).max().unwrap();
    let mut horiz_patterns: Vec<Vec<(usize, usize)>> = Vec::new();
    // base
    horiz_patterns.push(pattern_offsets.clone());
    // flip lr
    horiz_patterns.push(
        pattern_offsets
            .iter()
            .map(|(x, y)| (pattern_w - x, *y))
            .collect(),
    );
    // flip tb
    horiz_patterns.push(
        pattern_offsets
            .iter()
            .map(|(x, y)| (*x, pattern_h - y))
            .collect(),
    );
    // flip both
    horiz_patterns.push(
        pattern_offsets
            .iter()
            .map(|(x, y)| (pattern_w - x, pattern_h - y))
            .collect(),
    );
    let mut vert_patterns: Vec<Vec<(usize, usize)>> = Vec::new();
    // rot
    vert_patterns.push(pattern_offsets.iter().map(|(x, y)| (*y, *x)).collect());
    // rot flip lr
    vert_patterns.push(
        pattern_offsets
            .iter()
            .map(|(x, y)| (*y, pattern_w - x))
            .collect(),
    );
    // rot flip tb
    vert_patterns.push(
        pattern_offsets
            .iter()
            .map(|(x, y)| (pattern_h - y, *x))
            .collect(),
    );
    // rot flip both
    vert_patterns.push(
        pattern_offsets
            .iter()
            .map(|(x, y)| (pattern_h - y, pattern_w - x))
            .collect(),
    );

    let mut found_patterns: Vec<(usize, usize)> = Vec::new();
    for y in 0..packed_dim {
        for x in 0..packed_dim {
            let base_idx = y * packed_dim + x;
            // check horizontal patterns
            if x + pattern_w + 1 < packed_dim && y + pattern_h + 1 < packed_dim {
                for hp in horiz_patterns.iter() {
                    if hp.iter().all(|(px, py)| {
                        let pattern_linear_offset = px + py * packed_dim;
                        final_grid[base_idx + pattern_linear_offset] == '#'
                    }) {
                        found_patterns.push((x, y));
                    }
                }
            }
            // check  vertical patterns
            if x + pattern_h + 1 < packed_dim && y + pattern_w + 1 < packed_dim {
                for vp in vert_patterns.iter() {
                    if vp.iter().all(|(x, y)| {
                        let pattern_linear_offset = x + y * packed_dim;
                        final_grid[base_idx + pattern_linear_offset] == '#'
                    }) {
                        found_patterns.push((x, y));
                    }
                }
            }
        }
    }
    println!("found patterns at : {:?}", found_patterns);

    let total_water = final_grid.iter().filter(|&&x| x == '#').count();
    let non_pattern_water = total_water - pattern_offsets.len() * found_patterns.len();
    println!("p2 non monster water: {}", non_pattern_water);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("We need exactly one arguemnt, got: {}", args.len() - 1);
    }

    let file_contents = fs::read_to_string(&args[1]).expect("couldn't read the file");
    let tiles = parse(&file_contents);

    align_grid(&tiles);
}
