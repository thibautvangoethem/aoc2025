use std::{
    collections::{HashSet, VecDeque},
    fs,
    str::FromStr,
};

use itertools::Itertools;

pub fn solve(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    solve1(path)?;
    solve2(path)?;
    Ok(())
}

#[derive(Debug, Clone, PartialEq)]
struct Machine {
    lights: Vec<bool>,
    button_wiring: Vec<Vec<bool>>,
    joltage_req: Vec<i32>,
}

#[derive(Clone, Debug)]
struct ParseError;
impl FromStr for Machine {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, ParseError> {
        let vals: Vec<&str> = s.trim().split(" ").collect();
        let mut lights: Vec<bool> = Vec::new();
        let mut wiring: Vec<Vec<bool>> = Vec::new();
        let mut jolt: Vec<i32> = Vec::new();
        for handling in vals {
            let mut c_iter = handling.chars();
            let first_char: char = c_iter.next().unwrap();
            if first_char == '[' {
                while let Some(val) = c_iter.next() {
                    if val == '#' {
                        lights.push(true);
                    } else if val == '.' {
                        lights.push(false);
                    }
                }
            } else if first_char == '(' {
                let mut subwiring: Vec<bool> = vec![false; lights.len()];
                while let Some(val) = c_iter.next() {
                    if val != ',' && val != ')' {
                        let i = val.to_digit(10).unwrap();
                        subwiring[i as usize] = true
                    }
                }
                wiring.push(subwiring);
            } else if first_char == '{' {
                //Cool never had seen this next_back functions
                c_iter.next_back();
                let restval: &str = c_iter.as_str();
                for i in restval.split(',') {
                    jolt.push(i.parse().unwrap());
                }
            } else {
                panic!("this should not happen")
            }
        }

        Ok(Machine {
            lights: lights,
            button_wiring: wiring,
            joltage_req: jolt,
        })
    }
}

#[derive(Clone, Debug)]
struct Visiting {
    state: Vec<bool>,
    visited: HashSet<usize>,
}

fn apply_vec(state: &Vec<bool>, apply: &Vec<bool>) -> Vec<bool> {
    state
        .iter()
        .zip(apply.iter())
        .map(|(val1, val2)| val1 ^ val2)
        .collect()
}

fn bfs_search_machine(machine: &Machine) -> usize {
    let mut searching: VecDeque<Visiting> = VecDeque::from([Visiting {
        state: vec![false; machine.lights.len()],
        visited: HashSet::new(),
    }]);
    while let Some(handling) = searching.pop_front() {
        for (idx, val) in machine.button_wiring.iter().enumerate() {
            if !handling.visited.contains(&idx) {
                let mut newvisit = handling.visited.clone();
                newvisit.insert(idx.clone());
                let newstate = apply_vec(&handling.state, val);
                if newstate == machine.lights {
                    return newvisit.len();
                }
                searching.push_back(Visiting {
                    state: newstate,
                    visited: newvisit,
                });
            }
        }
    }
    panic!("it should have finished at this points");
}

pub fn solve1(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let binding = fs::read_to_string(path).expect("Should have been able to read the file");
    let race_against = binding
        .lines()
        .map(|line| Machine::from_str(line).unwrap())
        .collect::<Vec<Machine>>();
    println!(
        "the result for day10:1 is {0:?}",
        race_against
            .iter()
            .fold(0, |acc, machine| acc + bfs_search_machine(machine))
    );
    Ok(())
}
pub fn solve2(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
