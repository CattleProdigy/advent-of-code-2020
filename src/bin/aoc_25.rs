fn main() {
    let mut loop_count: usize = 1;
    let sub: usize = 7;
    let mut val: usize = 1;
    let card_pub = 3248366;
    loop {
        let new = (sub * val) % 20201227;
        if new == card_pub {
            break;
        }
        val = new;
        loop_count += 1;
    }
    println!("LC for pub key a: {}", loop_count);

    let sub: usize = 4738476; // pub key B
    let mut val: usize = 1;
    for _ in 0..loop_count {
        // loop size for A
        let new = (sub * val) % 20201227;
        val = new;
    }
    println!("enc: {}", val);
}
