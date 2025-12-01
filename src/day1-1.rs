use std::fs;
fn main() {
    let content = fs::read_to_string("dat/day1").unwrap();
    let lines: Vec<&str> = content.lines().collect();
    let res_class = 100;
    let mut zero_count = 0;
    let mut pos: i32 = 50;

    for idx in 0..lines.len() {
        let (dir, num_str) = lines[idx].split_at(1);
        let distance = num_str.parse::<i32>().unwrap();

        let delta = match dir {
            "L" => -1 * distance,
            "R" => 1 * distance,
            _ => panic!("not 2d!"),
        };

        pos += delta;

        pos %= res_class;
        if pos.is_negative() {
            pos += res_class;
        }

        if pos == 0 {
            zero_count += 1;
        }
    }

    println!("Zero count is {}", zero_count)
}
