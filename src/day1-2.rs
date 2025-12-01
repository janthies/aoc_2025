use std::fs;
fn main() {
    let content = fs::read_to_string("2025_day1.dat").unwrap();
    let lines: Vec<&str> = content.lines().collect();
    let res_class = 100;
    let mut zero_count = 0;
    let mut pos: i32 = 50;

    for idx in 0..lines.len() {
        let (dir, num_str) = lines[idx].split_at(1);
        let distance = num_str.parse::<i32>().unwrap();

        let delta = match dir {
            "L" => -1,
            "R" => 1,
            _ => panic!("not 2d!"),
        };

        print!("Start {}, ", pos);
        for _ in 0..distance.abs() {
            pos += delta;
            if pos % res_class == 0 {
                zero_count += 1;
            }
        }

        pos %= res_class;
        if pos.is_negative() {
            pos += res_class;
        }

        println!(
            "End {}, Steps {}, zero_count is {}",
            pos,
            delta * distance,
            zero_count
        );
    }

    println!("Zero count is {}", zero_count)
}
