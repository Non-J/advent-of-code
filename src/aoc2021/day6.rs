const INPUT: &str = include_str!("data/day6.txt");

pub fn entry() {
    println!("{}", run_part2(INPUT));
}

fn run_part1(input: &str) -> u64 {
    let mut state: [usize; 9] = [0; 9];
    input
        .split(",")
        .for_each(|x| state[x.parse::<usize>().unwrap()] += 1);

    for _ in 0..80 {
        let mut new_state: [usize; 9] = [0; 9];
        for i in 0..8 {
            new_state[i] = state[i + 1];
        }
        new_state[6] += state[0];
        new_state[8] += state[0];
        state = new_state;
    }

    state.iter().sum::<usize>() as u64
}

fn run_part2(input: &str) -> u64 {
    let mut state: [usize; 9] = [0; 9];
    input
        .split(",")
        .for_each(|x| state[x.parse::<usize>().unwrap()] += 1);

    for _ in 0..256 {
        let mut new_state: [usize; 9] = [0; 9];
        for i in 0..8 {
            new_state[i] = state[i + 1];
        }
        new_state[6] = new_state[6].checked_add(state[0]).unwrap();
        new_state[8] = new_state[8].checked_add(state[0]).unwrap();
        state = new_state;
    }

    state.iter().sum::<usize>() as u64
}

#[test]
fn example_test_case_1() {
    let input: &str = "3,4,3,1,2";
    assert_eq!(run_part1(input), 5934)
}

#[test]
fn example_test_case_2() {
    let input: &str = "3,4,3,1,2";
    assert_eq!(run_part2(input), 26984457539)
}
