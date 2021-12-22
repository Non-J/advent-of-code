use itertools::Itertools;

const INPUT: &str = include_str!("data/day1.txt");

pub fn entry() {
    println!("{}", run_part2(INPUT));
}

fn run_part1(input: &str) -> u64 {
    input
        .split("\n")
        .map(|x| x.parse::<u64>().expect("Not a number"))
        .tuple_windows()
        .filter(|(a, b)| a < b)
        .count() as u64
}

fn run_part2(input: &str) -> u64 {
    input
        .split("\n")
        .map(|x| x.parse::<u64>().expect("Not a number"))
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .filter(|(a, b)| a < b)
        .count() as u64
}

#[test]
fn example_test_case_1() {
    let input: &str = "199
200
208
210
200
207
240
269
260
263";
    assert_eq!(run_part1(input), 7)
}

#[test]
fn example_test_case_2() {
    let input: &str = "199
200
208
210
200
207
240
269
260
263";
    assert_eq!(run_part2(input), 5)
}

