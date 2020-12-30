struct Cups1 {
    cups: [usize; 9],
    current_cup_idx: usize,
}

fn wrap(mut x: i64) -> i64 {
    x = (x - 1) % 9;
    if x < 0 {
        10 + x
    } else {
        1 + x
    }
}

impl Cups1 {
    fn round(&mut self) {
        let pulled_cup_indices: [usize; 3] = [
            (self.current_cup_idx + 1) % 9,
            (self.current_cup_idx + 2) % 9,
            (self.current_cup_idx + 3) % 9,
        ];
        let cups_pulled: [usize; 3] = [
            self.cups[pulled_cup_indices[0]],
            self.cups[pulled_cup_indices[1]],
            self.cups[pulled_cup_indices[2]],
        ];

        let destination_idx = {
            let mut current_label = self.cups[self.current_cup_idx] - 1;
            if current_label == 0 {
                current_label = 9;
            }
            let mut maybe_dest_idx = self.cups.iter().position(|&c| c == current_label).unwrap();
            loop {
                if pulled_cup_indices.iter().any(|&x| x == maybe_dest_idx) {
                    current_label = current_label - 1;
                    if current_label == 0 {
                        current_label = 9;
                    }
                } else {
                    break;
                }
                maybe_dest_idx = self.cups.iter().position(|&c| c == current_label).unwrap();
            }
            maybe_dest_idx + 1
        };
        println!("dest {}", destination_idx);

        // slide down
        let mut idx = pulled_cup_indices[0];
        while idx != destination_idx % 9 {
            self.cups[idx] = self.cups[(idx + 3) % 9];
            idx = (idx + 1) % 9
        }
        println!("s:{:?}", self.cups);

        // write destination
        for i in 0..3 {
            let dest = (wrap(destination_idx as i64 - 3 + i as i64) % 9) as usize;
            println!("{}", dest);
            self.cups[dest] = cups_pulled[i];
        }
        println!("w:{:?}", self.cups);

        // pick current cup
        self.current_cup_idx = (self.current_cup_idx + 1) % 9;
    }
}

fn main() {
    {
        let mut example = Cups1 {
            cups: [3, 8, 9, 1, 2, 5, 4, 6, 7],
            current_cup_idx: 0,
        };

        for _ in 0..10 {
            println!("{:?}", example.cups);
            example.round();
        }
        println!("{:?}", example.cups);
    }
    {
        let mut example = Cups1 {
            cups: [5, 8, 3, 9, 7, 6, 2, 4, 1],
            current_cup_idx: 0,
        };

        for _ in 0..100 {
            println!("{:?}", example.cups);
            example.round();
        }
        println!("{:?}", example.cups);
    }
    {
        let mut v: Vec<usize> = vec![usize::MAX, 10, 4, 9, 1, 8, 2, 6, 3, 7];
        for i in 10..1000000 {
            v.push(i + 1);
        }
        v.push(5);

        let mut current_cup_label = 5;

        for i in 0..10000001 {
            println!("{}", i);
            let cups_pulled: [usize; 3] = [
                v[current_cup_label], // random access
                v[v[current_cup_label]],
                v[v[v[current_cup_label]]],
            ];
            let dest_label = {
                let mut dest_label = current_cup_label - 1;
                if dest_label == 0 {
                    dest_label = 1000000;
                }

                while cups_pulled.iter().any(|&x| x == dest_label) {
                    dest_label = dest_label - 1;
                    if dest_label == 0 {
                        dest_label = 1000000;
                    }
                }
                dest_label
            };

            // make the current cup point to the cup just
            // past the ones we pulled
            v[current_cup_label] = v[cups_pulled[2]];

            // left of destination points to first pulled cup
            let next_after_dest = v[dest_label];
            v[dest_label] = cups_pulled[0];

            // last pulled cup points to next after destination
            v[cups_pulled[2]] = next_after_dest;

            current_cup_label = v[current_cup_label];
        }

        let cup_after_1 = v[1];
        let cup_after_after_1 = v[v[1]];
        println!(
            "{} {} {}",
            cup_after_1,
            cup_after_after_1,
            cup_after_1 * cup_after_after_1
        );
    }
}
