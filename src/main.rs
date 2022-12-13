mod utils;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day08;
//mod day07;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;


fn main() {
    /*
    day01::answer_part_1("res/day01/part1.sample");
    day01::answer_part_1("res/day01/part1.real");
    day01::answer_part_2("res/day01/part1.sample");
    day01::answer_part_2("res/day01/part1.real");
    day02::answer_part_1("res/day02/input.sample");
    day02::answer_part_1("res/day02/input.real");
    day02::answer_part_2("res/day02/input.sample");
    day02::answer_part_2("res/day02/input.real");
    day03::answer_part_1("res/day03/input.sample");
    day03::answer_part_1("res/day03/input.real");
    day03::answer_part_2("res/day03/input.sample");
    day03::answer_part_2("res/day03/input.real");
    day04::answer_part_1("res/day04/input.sample");
    day04::answer_part_1("res/day04/input.real");
    day04::answer_part_2("res/day04/input.sample");
    day04::answer_part_2("res/day04/input.real");
    day05::answer_part_1("res/day05/input.sample");
    day05::answer_part_1("res/day05/input.real");
    day05::answer_part_2("res/day05/input.sample");
    day05::answer_part_2("res/day05/input.real");
    */
    println!("day06 part 1 sample answer: {:?}", day06::answer_part_1("res/day06/input.sample"));
    println!("day06 part 1 real answer  : {:?}", day06::answer_part_1("res/day06/input.real"));
    println!("day06 part 2 sample answer: {:?}", day06::answer_part_2("res/day06/input.sample"));
    println!("day06 part 2 real answer  : {:?}", day06::answer_part_2("res/day06/input.real"));
    //println!("day07 part 1 sample answer: {:?}", day07::answer_part_1("res/day07/input.sample"));
    //println!("day07 part 1 real answer  : {:?}", day07::answer_part_1("res/day07/input.real"));
    //println!("day07 part 2 sample answer: {:?}", day07::answer_part_2("res/day07/input.sample"));
    //println!("day07 part 2 real answer  : {:?}", day07::answer_part_2("res/day07/input.real"));
    println!("day08 part 1 sample answer: {:?}", day08::answer_part_1("res/day08/input.sample"));
    //println!("day08 part 1 real answer  : {:?}", day08::answer_part_1("res/day08/input.real"));
    println!("day08 part 2 sample answer: {:?}", day08::answer_part_2("res/day08/input.sample"));
    println!("day08 part 2 real answer  : {:?}", day08::answer_part_2("res/day08/input.real"));
    println!("day09 part 1 sample answer: {:?}", day09::answer_part_1("res/day09/input.sample"));
    println!("day09 part 1 real answer  : {:?}", day09::answer_part_1("res/day09/input.real"));
    println!("day09 part 2 sample answer: {:?}", day09::answer_part_2("res/day09/input.sample"));
    println!("day09 part 2 sample answer2: {:?}", day09::answer_part_2("res/day09/input.sample2"));
    println!("day09 part 2 real answer  : {:?}", day09::answer_part_2("res/day09/input.real"));

    println!("day10 part 1 sample answer: {}", day10::answer_part_1("res/day10/input.sample"));
    println!("day10 part 1 real answer  : {}", day10::answer_part_1("res/day10/input.real"));
    println!("day10 part 2 sample answer: \n{}", day10::answer_part_2("res/day10/input.sample"));
    println!("day10 part 2 real answer  : \n{}", day10::answer_part_2("res/day10/input.real"));

    println!("day11 part 1 sample answer: {}", day11::answer_part_1("res/day11/input.sample"));
    println!("day11 part 1 real answer  : {}", day11::answer_part_1("res/day11/input.real"));
    println!("day11 part 2 sample answer: {}", day11::answer_part_2("res/day11/input.sample"));
    println!("day11 part 2 real answer  : {}", day11::answer_part_2("res/day11/input.real"));

    println!("day12 part 1 sample answer: {}", day12::answer_part_1("res/day12/input.sample"));
    println!("day12 part 1 real answer  : {}", day12::answer_part_1("res/day12/input.real"));
    println!("day12 part 2 sample answer: {}", day12::answer_part_2("res/day12/input.sample"));
    println!("day12 part 2 real answer  : {}", day12::answer_part_2("res/day12/input.real"));

    println!("day13 part 1 sample answer: {}", day13::answer_part_1(day13::data1()));
    println!("day13 part 1 real answer  : {}", day13::answer_part_1(day13::data2()));
    println!("day13 part 2 sample answer: {}", day13::answer_part_2(day13::data1()));
    println!("day13 part 2 real answer  : {}", day13::answer_part_2(day13::data2()));
}
