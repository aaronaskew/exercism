use std::collections::{BTreeSet, HashMap, HashSet};

use itertools::Itertools;

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let leading_letters = input
        .split(&['+', '='])
        .filter_map(|word| {
            let trimmed = word.trim();

            if trimmed.len() < 2 {
                None
            } else {
                trimmed.chars().next()
            }
        })
        .collect::<HashSet<_>>();

    let mut words = input
        .split(&['+', '='])
        .filter_map(|word| {
            let trimmed_word = word.trim();
            match trimmed_word.len() {
                0 => None,
                _ => Some(trimmed_word),
            }
        })
        .collect::<Vec<_>>();

    let all_letters = words
        .iter()
        .flat_map(|word| word.chars())
        .collect::<BTreeSet<_>>();

    let sum = words.pop().expect("sum should exist");

    let addends = words;

    (0..=9u8)
        .permutations(all_letters.len())
        .filter_map(|perm| {
            let possible_solutions = all_letters
                .clone()
                .into_iter()
                .zip(perm.clone())
                .collect::<HashMap<_, _>>();

            if perm.contains(&0) {
                let key = possible_solutions
                    .iter()
                    .find(|(_, &val)| val == 0)
                    .map(|(&key, _)| key)
                    .expect("letter equaling zero exists");

                if leading_letters.contains(&key) {
                    return None;
                }
            }

            Some(possible_solutions)
        })
        .find(|possible_solution| is_valid_solution(possible_solution, addends.clone(), sum))
}

fn is_valid_solution(candidate: &HashMap<char, u8>, addends: Vec<&str>, sum: &str) -> bool {
    addends
        .iter()
        .map(|&addend| letters_to_number(candidate, addend))
        .sum::<u64>()
        == letters_to_number(candidate, sum)
}

fn letters_to_number(candidate: &HashMap<char, u8>, letters: &str) -> u64 {
    letters
        .chars()
        .rev()
        .enumerate()
        .map(|(i, c)| {
            (*candidate
                .get(&c)
                .expect("candidate should include this letter") as u64)
                * 10u64.pow(i as u32)
        })
        .sum()
}
