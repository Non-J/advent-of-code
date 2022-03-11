use itertools::Itertools;

const INPUT: &str = include_str!("data/day8.txt");

const DIGITS_MAP: [&str; 10] = [
    "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
];

#[derive(Clone)]
struct WireMap {
    map: [char; 7],
}

impl WireMap {
    pub fn new() -> Self {
        Self {
            map: ['*', '*', '*', '*', '*', '*', '*'],
        }
    }

    // Map the incorrect wiring to correct ones
    fn convert(&self, input: &str) -> String {
        input
            .chars()
            .map(|x| match x {
                'a' => self.map[0],
                'b' => self.map[1],
                'c' => self.map[2],
                'd' => self.map[3],
                'e' => self.map[4],
                'f' => self.map[5],
                'g' => self.map[6],
                _ => '*',
            })
            .collect()
    }
}

pub fn entry() {
    println!("{}", run_part2(INPUT));
}

fn run_part1(input: &str) -> u64 {
    let lines = input
        .split('\n')
        .map(|line| {
            let (s, q) = line.split_once(" | ").unwrap();
            (
                s.split_whitespace().collect_vec(),
                q.split_whitespace().collect_vec(),
            )
        })
        .collect_vec();

    lines
        .iter()
        .map(|(_, queries)| {
            queries
                .iter()
                .map(|x| x.len())
                .filter(|&x| x == 2 || x == 4 || x == 3 || x == 7)
                .count()
        })
        .sum::<usize>() as u64
}

fn run_part2(input: &str) -> u64 {
    let lines = input
        .split('\n')
        .map(|line| {
            let (s, q) = line.split_once(" | ").unwrap();
            (
                s.split_whitespace().collect_vec(),
                q.split_whitespace().collect_vec(),
            )
        })
        .collect_vec();

    let mut sum: u64 = 0;

    for (samples, queries) in lines {
        let mut mapping = vec![String::new(); 10];
        let mut len5 = Vec::<String>::new();
        let mut len6 = Vec::<String>::new();

        for x in samples {
            match x.len() {
                2 => mapping[1] = x.chars().sorted().collect(),
                3 => mapping[7] = x.chars().sorted().collect(),
                4 => mapping[4] = x.chars().sorted().collect(),
                5 => len5.push(x.chars().sorted().collect()),
                6 => len6.push(x.chars().sorted().collect()),
                7 => mapping[8] = x.chars().sorted().collect(),
                _ => {
                    panic!("Invalid input")
                }
            }
        }

        let correct_mapping = len5
            .iter()
            .permutations(3)
            .cartesian_product(len6.iter().permutations(3))
            .map(|(len5, len6)| {
                let mut mapping = mapping.clone();
                mapping[2] = len5[0].clone();
                mapping[3] = len5[1].clone();
                mapping[5] = len5[2].clone();
                mapping[0] = len6[0].clone();
                mapping[6] = len6[1].clone();
                mapping[9] = len6[2].clone();
                mapping
            })
            .find(|mapping| {
                let union = |a: &str, b: &str| -> String {
                    a.chars().chain(b.chars()).unique().sorted().collect()
                };

                // Filter out incorrect len5

                // Union 3 and 1 should yield 3 whereas 2 and 5 doesn't; confirms 3
                if union(mapping[3].as_str(), mapping[1].as_str()) != mapping[3] {
                    return false;
                }

                // Union of 2 and 4 should yield 8 whereas 5 doesn't; confirms 2 and 5
                if union(mapping[2].as_str(), mapping[4].as_str()) != mapping[8] {
                    return false;
                }

                // Filter out incorrect len6

                // Union of 1 and 6 should yeild 8 whereas 0 and 9 doesn't; comfirms 6
                if union(mapping[1].as_str(), mapping[6].as_str()) != mapping[8] {
                    return false;
                }

                // Union of 0 and 4 should yeild 8 whereas 9 doesn't; comfirms 0 and 9
                if union(mapping[0].as_str(), mapping[4].as_str()) != mapping[8] {
                    return false;
                }

                true
            })
            .unwrap();

        let result = queries
            .iter()
            .map(|query| query.chars().sorted().collect::<String>())
            .map(|query| {
                correct_mapping
                    .iter()
                    .enumerate()
                    .find(|(_, m)| **m == query)
                    .map(|(idx, _)| idx as u64)
                    .unwrap()
            })
            .collect_vec();

        sum = sum
            .checked_add(result[0] * 1000 + result[1] * 100 + result[2] * 10 + result[3])
            .unwrap();
    }

    sum
}

#[test]
fn example_test_case_1() {
    let input: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
    assert_eq!(run_part1(input), 26)
}

#[test]
fn example_test_case_2() {
    let input: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
    assert_eq!(run_part2(input), 61229)
}
