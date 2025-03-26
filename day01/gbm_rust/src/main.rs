use std::env;
use std::fs;

fn count_floor(instructions: String) -> (i32, i32){
    let mut floor: i32 = 0;
    let mut counter: i32 = 1;
    let mut basement:i32 = -1;
    for c in instructions.chars() {
        match c{
            '('=> floor += 1,
            ')'=> floor -= 1,
            _=> ()
        }
        if basement < 0 && floor < 0 {
            basement = counter;
        }
        counter += 1;
    }
    return (floor, basement);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let input = fs::read_to_string(file_path)
        .expect("Need to be able to read the file");

    println!("Read in:\n{input}");

    let (part1, part2) = count_floor(input);

    println!("Part 1: {part1}\nPart 2: {part2}");
}
