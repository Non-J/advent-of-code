use itertools::Itertools;
use std::cmp::min;
use std::collections::BTreeMap;

const INPUT: &str = include_str!("data/day7.txt");

pub fn entry() {
    println!("{}", run_part2(INPUT));
}

fn run_part1(input: &str) -> u64 {
    let mut counter = BTreeMap::new();

    let mut before = 0;
    let mut after = 0;

    input
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .for_each(|x| {
            after += 1;
            counter.entry(x).and_modify(|val| *val += 1).or_insert(1);
        });

    let mut init_fuel: i64 = 0;
    let mut delta: i64 = 0;
    let mut lowest_delta: i64 = 0;
    let mut prev_pos: i64 = 0;
    let mut init_pos: i64 = 0;

    let mut iter = counter.iter();

    if let Some((pos, cnt)) = iter.next() {
        prev_pos = *pos;
        init_pos = *pos;
        after -= cnt;
        before += cnt;
    }

    for (pos, cnt) in iter {
        init_fuel += cnt * (pos - init_pos);

        delta += before * (pos - prev_pos);
        delta -= after * (pos - prev_pos);
        lowest_delta = min(lowest_delta, delta);

        before += cnt;
        after -= cnt;
        prev_pos = *pos;
    }

    (init_fuel + lowest_delta) as u64
}

fn run_part2(input: &str) -> u64 {
    let mut costs: Vec<u64> = vec![0, 1, 3, 6, 10];

    let mut get_cost = |dist: i64| -> i64 {
        let dist = dist as usize;

        if let Some(&result) = costs.get(dist) {
            result.try_into().unwrap()
        } else {
            let start = costs.len();
            costs.resize(dist + 1, 0);
            for i in start..=dist {
                costs[i] = costs[i - 1] + i as u64;
            }
            (*costs.last().unwrap()).try_into().unwrap()
        }
    };

    let mut counter = BTreeMap::new();

    input
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .for_each(|x| {
            counter.entry(x).and_modify(|val| *val += 1).or_insert(1);
        });

    let entries = counter.iter().collect_vec();

    let mut lowest_fuel: i64 = i64::MAX;

    let range = (*entries.first().unwrap().0, *entries.last().unwrap().0);
    for cur_pos in range.0..=range.1 {
        let mut fuel: i64 = 0;

        for (&pos, &cnt) in entries.iter() {
            if pos < cur_pos {
                fuel += cnt * get_cost(cur_pos - pos)
            } else if pos > cur_pos {
                fuel += cnt * get_cost(pos - cur_pos)
            }
        }

        lowest_fuel = min(lowest_fuel, fuel);
    }

    lowest_fuel as u64
}

#[test]
fn example_test_case_1() {
    let input: &str = "16,1,2,0,4,2,7,1,2,14";
    assert_eq!(run_part1(input), 37)
}

#[test]
fn example_test_case_2() {
    let input: &str = "16,1,2,0,4,2,7,1,2,14";
    assert_eq!(run_part2(input), 168)
}
