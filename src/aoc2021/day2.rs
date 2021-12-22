const INPUT: &str = include_str!("data/day2.txt");

pub fn entry() {
    println!("{}", run_part2(INPUT));
}

fn run_part1(input: &str) -> u64 {
    let mut horizontal = 0;
    let mut depth = 0;

    input
        .split("\n")
        .map(|x| {
            x.split_once(" ")
                .map(|x| (x.0, x.1.parse::<u64>().expect("Not a number")))
                .expect("Bad text format")
        })
        .for_each(|(cmd, val)| {
            if cmd == "forward" {
                horizontal += val
            } else if cmd == "down" {
                depth += val
            } else if cmd == "up" {
                depth -= val
            }
        });

    horizontal * depth
}

fn run_part2(input: &str) -> u64 {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    input
        .split("\n")
        .map(|x| {
            x.split_once(" ")
                .map(|x| (x.0, x.1.parse::<u64>().expect("Not a number")))
                .expect("Bad text format")
        })
        .for_each(|(cmd, val)| {
            if cmd == "forward" {
                horizontal += val;
                depth += aim * val;
            } else if cmd == "down" {
                aim += val
            } else if cmd == "up" {
                aim -= val
            }
        });

    horizontal * depth
}

#[test]
fn example_test_case_1() {
    let input: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";
    assert_eq!(run_part1(input), 150)
}

#[test]
fn example_test_case_2() {
    let input: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";
    assert_eq!(run_part2(input), 900)
}
