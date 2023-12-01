use aoc2023::read_as_lines;

fn main() {
    let mut input = read_as_lines("../inputs/day_1/input");
    let part_1 = day_1::get_total_calibration(&mut input);
    println!("Part 1\n-------------------\n{part_1}");
    
    let mut input = read_as_lines("../inputs/day_1/input");
    let part_2 = day_1::get_total_calibration_2(&mut input);
    println!("Part 1\n-------------------\n{part_2}");
}

