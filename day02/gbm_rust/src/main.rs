use std::env;
use std::fs;

fn gift_wrapping_square_footage(l: i32, w: i32, h: i32) -> i32 {
    2 * l * w + 2 * w * h + 2 * h * l + [l * w, w * h, h * l].iter().min().unwrap()
}

fn gift_ribbon_footage(l: i32, w: i32, h: i32) -> i32 {
    let mut s = [l,w,h];
    s.sort();
    2 * s[0] + 2 * s[1] + l * w * h
}

fn gift_list_square_footage(gift_list: String) -> (i32, i32) {
    let mut total_wrapping: i32 = 0;
    let mut total_ribbon: i32 = 0;
    for gift in gift_list.lines() {
        let dim: Vec<i32> = gift
            .split("x")
            .map(|x| x.parse().unwrap_or_default())
            .collect();
        total_wrapping += gift_wrapping_square_footage(dim[0], dim[1], dim[2]);
        total_ribbon += gift_ribbon_footage(dim[0], dim[1], dim[2]);
    }
    return (total_wrapping, total_ribbon);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let input = fs::read_to_string(file_path).expect("Need to be able to read the file");

    let (part1, part2) = gift_list_square_footage(input);
    println!("Part 1: {part1}\nPart 2: {part2}");
}
