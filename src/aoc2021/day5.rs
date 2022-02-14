use itertools::Itertools;
use std::cmp::max;

const INPUT: &str = include_str!("data/day5.txt");

pub fn entry() {
    println!("{}", run_part2(INPUT));
}

pub fn minmax<T>(a: T, b: T) -> (T, T)
where
    T: std::cmp::PartialOrd,
{
    if a <= b {
        return (a, b);
    } else {
        return (b, a);
    }
}

enum Direction {
    Vertical,
    Horizontal,
    DiagonalD,
    DiagonalU,
}

struct Entry {
    begin_x: u64,
    begin_y: u64,
    end_x: u64,
    end_y: u64,
}

impl Entry {
    fn parse(input: &str) -> Self {
        let parse_pair = |(x, y): (&str, &str)| {
            (
                x.parse::<u64>().expect("Not a number"),
                y.parse::<u64>().expect("Not a number"),
            )
        };

        let (begin, end) = input.split_once(" -> ").expect("Invalid input");
        let (begin_x, begin_y) = begin
            .split_once(",")
            .map(parse_pair)
            .expect("Invalid input");
        let (end_x, end_y) = end.split_once(",").map(parse_pair).expect("Invalid input");

        Entry {
            begin_x,
            begin_y,
            end_x,
            end_y,
        }
    }

    fn get_direction(&self) -> Direction {
        if self.begin_x == self.end_x {
            return Direction::Vertical;
        } else if self.begin_y == self.end_y {
            return Direction::Horizontal;
        } else if self.begin_y < self.end_y && self.begin_x < self.end_x {
            return Direction::DiagonalD;
        } else if self.begin_y < self.end_y && self.begin_x > self.end_x {
            return Direction::DiagonalU;
        } else if self.begin_y > self.end_y && self.begin_x < self.end_x {
            return Direction::DiagonalU;
        } else if self.begin_y > self.end_y && self.begin_x > self.end_x {
            return Direction::DiagonalD;
        }

        // The above conditions should over all possible cases
        unreachable!();
    }

    fn max(&self) -> u64 {
        max(max(self.begin_x, self.begin_y), max(self.end_x, self.end_y))
    }
}

fn run_part1(input: &str) -> u64 {
    let entries = input
        .split("\n")
        .map(|entry| Entry::parse(entry))
        .collect_vec();

    let max_size = entries
        .iter()
        .map(|entry| entry.max())
        .max()
        .expect("No entry (input is empty)") as usize;

    let mut field = vec![vec![0 as u64; max_size + 1]; max_size + 1];

    for entry in entries {
        match entry.get_direction() {
            Direction::Vertical => {
                let (begin, end) = minmax(entry.begin_y, entry.end_y);
                for y in begin..=end {
                    field[y as usize][entry.begin_x as usize] += 1;
                }
            }
            Direction::Horizontal => {
                let (begin, end) = minmax(entry.begin_x, entry.end_x);
                for x in begin..=end {
                    field[entry.begin_y as usize][x as usize] += 1;
                }
            }
            Direction::DiagonalD | Direction::DiagonalU => {}
        }
    }

    field.iter().flatten().filter(|&&x| x > 1).count() as u64
}

fn run_part2(input: &str) -> u64 {
    let entries = input
        .split("\n")
        .map(|entry| Entry::parse(entry))
        .collect_vec();

    let max_size = entries
        .iter()
        .map(|entry| entry.max())
        .max()
        .expect("No entry (input is empty)") as usize;

    let mut field = vec![vec![0 as u64; max_size + 1]; max_size + 1];

    for entry in entries {
        match entry.get_direction() {
            Direction::Vertical => {
                let (begin, end) = minmax(entry.begin_y, entry.end_y);
                for y in begin..=end {
                    field[y as usize][entry.begin_x as usize] += 1;
                }
            },
            Direction::Horizontal => {
                let (begin, end) = minmax(entry.begin_x, entry.end_x);
                for x in begin..=end {
                    field[entry.begin_y as usize][x as usize] += 1;
                }
            },
            Direction::DiagonalD => {
                let (start_x, start_y, dist) = if entry.begin_x <= entry.end_x {
                    (entry.begin_x, entry.begin_y, entry.end_x - entry.begin_x)
                } else {
                    (entry.end_x, entry.end_y, entry.begin_x - entry.end_x)
                };

                for i in 0..=dist {
                    field[(start_y+i) as usize][(start_x+i) as usize] += 1;
                }
            },
            Direction::DiagonalU => {
                let (start_x, start_y, dist) = if entry.begin_x <= entry.end_x {
                    (entry.begin_x, entry.begin_y, entry.end_x - entry.begin_x)
                } else {
                    (entry.end_x, entry.end_y, entry.begin_x - entry.end_x)
                };

                for i in 0..=dist {
                    field[(start_y-i) as usize][(start_x+i) as usize] += 1;
                }
            },
        }
    }

    field.iter().flatten().filter(|&&x| x > 1).count() as u64
}

#[test]
fn example_test_case_1() {
    let input: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
    assert_eq!(run_part1(input), 5)
}

#[test]
fn example_test_case_2() {
    let input: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
    assert_eq!(run_part2(input), 12)
}
