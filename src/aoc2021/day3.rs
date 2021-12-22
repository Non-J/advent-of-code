const INPUT: &str = include_str!("data/day3.txt");

pub fn entry() {
    println!("{}", run_part2(INPUT));
}

fn run_part1(input: &str) -> u64 {
    let mut one_count = Vec::<u64>::new();
    let length = input.split("\n").count() as u64;

    for line in input.split("\n") {
        if one_count.len() < line.len() {
            one_count.resize(line.len(), 0);
        }

        for (idx, char) in line.chars().enumerate() {
            if char == '1' {
                one_count[idx] += 1
            }
        }
    }

    let mut gamma = 0;
    let mut epsilon = 0;

    for val in one_count {
        gamma *= 2;
        epsilon *= 2;

        if val > length / 2 {
            gamma += 1;
        } else {
            epsilon += 1;
        }
    }

    gamma * epsilon
}

#[derive(Clone)]
struct MaskType {
    o2: bool,
    co2: bool,
}

fn run_part2(input: &str) -> u64 {
    let line_length = input.split_once("\n").map_or(input.len(), |x| x.0.len());
    let line_count = input.split("\n").count();

    let mut o2_value = 0;
    {
        let mut masks = Vec::<bool>::with_capacity(line_count);
        masks.resize(line_count, true);

        for i in 0..line_length {
            // Count the number of set bit across the column
            let one_count = input
                .split("\n")
                .zip(masks.iter())
                .filter(|(line, &mask)| mask && line.as_bytes()[i] == b'1')
                .count();

            let is_one_most_common = one_count * 2 >= masks.iter().filter(|x| **x).count();

            // Update mask
            input
                .split("\n")
                .zip(masks.iter_mut())
                .filter(|(line, &mut mask)| {
                    mask && ((is_one_most_common && line.as_bytes()[i] == b'0')
                        || (!is_one_most_common && line.as_bytes()[i] == b'1'))
                })
                .for_each(|(_, mask)| {
                    *mask = false;
                });

            // Filter for result
            if masks.iter().filter(|&&x| x).count() == 1 {
                o2_value = input
                    .split("\n")
                    .zip(masks.iter())
                    .filter(|(_, &mask)| mask)
                    .nth(0)
                    .map(|(line, _)| {
                        u64::from_str_radix(line, 2).expect("It should be a binary string")
                    })
                    .expect("There's no input satisfying the condition");
            }
        }
    }

    // CO2 Value
    let mut co2_value = 0;
    {
        let mut masks = Vec::<bool>::with_capacity(line_count);
        masks.resize(line_count, true);

        for i in 0..line_length {
            // Count the number of set bit across the column
            let one_count = input
                .split("\n")
                .zip(masks.iter())
                .filter(|(line, &mask)| mask && line.as_bytes()[i] == b'1')
                .count();

            let is_one_least_common = one_count * 2 < masks.iter().filter(|x| **x).count();

            // Update mask
            input
                .split("\n")
                .zip(masks.iter_mut())
                .filter(|(line, &mut mask)| {
                    mask && ((is_one_least_common && line.as_bytes()[i] == b'0')
                        || (!is_one_least_common && line.as_bytes()[i] == b'1'))
                })
                .for_each(|(_, mask)| {
                    *mask = false;
                });

            // Filter for result
            if masks.iter().filter(|&&x| x).count() == 1 {
                co2_value = input
                    .split("\n")
                    .zip(masks.iter())
                    .filter(|(_, &mask)| mask)
                    .nth(0)
                    .map(|(line, _)| {
                        u64::from_str_radix(line, 2).expect("It should be a binary string")
                    })
                    .expect("There's no input satisfying the condition");
            }
        }
    }

    o2_value * co2_value
}

#[test]
fn example_test_case_1() {
    let input: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
    assert_eq!(run_part1(input), 198)
}

#[test]
fn example_test_case_2() {
    let input: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
    assert_eq!(run_part2(input), 230)
}
