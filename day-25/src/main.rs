fn run_encryption(subject_number: u32, loop_size: u32) -> u32 {
    let mut value = 1;
    for _ in 0..loop_size { value = (value as u64* subject_number as u64) % 20201227; }

    value as u32
}

fn derive_loop_number(public_key: u32) -> u32 {
    let mut attempt = 1;
    let mut loops = 0;

    while attempt != public_key {
        attempt = (attempt * 7) % 20201227;
        loops += 1;
    }

    loops
}

fn main() {
    // Part one
    let card_loop_number = derive_loop_number(1327981);
    println!("Part one: {}", run_encryption(2822615, card_loop_number));
}

#[test]
fn test_part_one() {
    assert_eq!(derive_loop_number(5764801), 8);
    assert_eq!(derive_loop_number(17807724), 11);
    assert_eq!(run_encryption(5764801, 11), 14897079);
    assert_eq!(run_encryption(17807724, 8), 14897079);
}
