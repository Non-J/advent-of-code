use itertools::Itertools;

const INPUT: &str = include_str!("data/day4.txt");

pub fn entry() {
    println!("{}", run_part2(INPUT));
}

const BOARD_SIZE: usize = 5;

struct Board {
    data: Vec<u8>,
    marker: Vec<bool>,
}

impl Board {
    fn from(data: Vec<u8>) -> Self {
        assert_eq!(BOARD_SIZE * BOARD_SIZE, data.len());
        Self {
            data,
            marker: vec![false; BOARD_SIZE * BOARD_SIZE],
        }
    }

    fn has_won(&self) -> bool {
        // True if any row is all marked
        for row in 0..BOARD_SIZE {
            if (0..BOARD_SIZE)
                .map(|col| self.marker[row * BOARD_SIZE + col])
                .all(|x| x)
            {
                return true;
            }
        }

        // True if any column is all marked
        for col in 0..BOARD_SIZE {
            if (0..BOARD_SIZE)
                .map(|row| self.marker[row * BOARD_SIZE + col])
                .all(|x| x)
            {
                return true;
            }
        }

        // False otherwise
        false
    }

    fn mark_number(&mut self, number: u8) {
        self.data
            .iter()
            .zip(self.marker.iter_mut())
            .filter(|(&x, _)| x == number)
            .for_each(|(_, mark)| *mark = true);
    }

    fn sum_unmarked_numbers(&self) -> u64 {
        self.data
            .iter()
            .zip(self.marker.iter())
            .filter(|(_, &mark)| !mark)
            .map(|(&item, _)| item as u64)
            .sum()
    }
}

fn run_part1(input: &str) -> u64 {
    let (input_numbers, input_boards) = input.split_once("\n\n").expect("Invalid input format");
    let numbers = input_numbers
        .split(",")
        .map(|x| x.parse::<u8>().expect("It should be a number"))
        .collect_vec();
    let mut boards = input_boards
        .split("\n\n")
        .map(|board| {
            // Spilt board by whitespace and convert each element to number
            board
                .split_whitespace()
                .map(|item| item.parse::<u8>().expect("It should be a number"))
                .collect_vec()
        })
        .map(|board| Board::from(board))
        .collect_vec();

    for number in numbers {
        boards
            .iter_mut()
            .for_each(|board| board.mark_number(number));

        if let Some(winning_board) = boards.iter().find(|board| board.has_won()) {
            return winning_board.sum_unmarked_numbers() * number as u64;
        }
    }

    // If the code is correct, then the function should return by the loop above
    // Thus, this line is unreachable
    unreachable!()
}

fn run_part2(input: &str) -> u64 {
    let (input_numbers, input_boards) = input.split_once("\n\n").expect("Invalid input format");
    let numbers = input_numbers
        .split(",")
        .map(|x| x.parse::<u8>().expect("It should be a number"))
        .collect_vec();
    let mut boards = input_boards
        .split("\n\n")
        .map(|board| {
            // Spilt board by whitespace and convert each element to number
            board
                .split_whitespace()
                .map(|item| item.parse::<u8>().expect("It should be a number"))
                .collect_vec()
        })
        .map(|board| Board::from(board))
        .collect_vec();

    let mut board_has_won_before = vec![false; boards.len()];
    let mut remaining_board = boards.len();
    let mut result = 0;

    for number in numbers {
        boards
            .iter_mut()
            .zip(board_has_won_before.iter_mut())
            .filter(|(_, &mut has_won_before)| !has_won_before)
            .for_each(|(board, has_won_before)| {
                board.mark_number(number);
                if board.has_won() {
                    *has_won_before = true;
                    remaining_board -= 1;
                    if remaining_board == 0 {
                        result = board.sum_unmarked_numbers() * number as u64;
                    }
                }
            });

        if remaining_board == 0 {
            break;
        }
    }

    result
}

#[test]
fn example_test_case_1() {
    let input: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
";
    assert_eq!(run_part1(input), 4512)
}

#[test]
fn example_test_case_2() {
    let input: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
";
    assert_eq!(run_part2(input), 1924)
}
