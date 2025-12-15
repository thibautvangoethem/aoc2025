use std::{collections::VecDeque, fs, str::FromStr};

pub fn solve(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    solve1(path)?;
    solve2(path)?;
    Ok(())
}

#[derive(Debug, Clone, PartialEq)]
struct Machine {
    lights: u16,
    button_wiring: Vec<u16>,
    joltage_req: Vec<i32>,
}

#[derive(Clone, Debug)]
struct ParseError;
impl FromStr for Machine {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, ParseError> {
        let vals: Vec<&str> = s.trim().split(" ").collect();
        let mut lights: u16 = 0;
        let mut wiring: Vec<u16> = Vec::new();
        let mut jolt: Vec<i32> = Vec::new();
        for handling in vals {
            let mut c_iter = handling.chars();
            let first_char: char = c_iter.next().unwrap();
            if first_char == '[' {
                let mut idx = 0;
                while let Some(val) = c_iter.next() {
                    if val == '#' {
                        lights ^= 1 << idx;
                    }
                    idx += 1;
                }
            } else if first_char == '(' {
                let mut subwiring: u16 = 0;
                while let Some(val) = c_iter.next() {
                    if val != ',' && val != ')' {
                        let i = val.to_digit(10).unwrap();
                        subwiring ^= 1 << i;
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
    state: u16,
    //used for bitwise operations
    visited: u16,
}
fn get_bit_at(input: &u16, n: &usize) -> bool {
    if *n < 16 {
        input & (1 << n) != 0
    } else {
        false
    }
}

fn bfs_search_machine(machine: &Machine) -> usize {
    let mut searching: VecDeque<Visiting> = VecDeque::from([Visiting {
        state: 0,
        visited: 0,
    }]);
    while let Some(handling) = searching.pop_front() {
        for (idx, val) in machine.button_wiring.iter().enumerate() {
            if !get_bit_at(&handling.visited, &idx) {
                let mut newvisit = handling.visited.clone();
                newvisit ^= 1 << idx;
                let newstate = handling.state ^ val;
                if newstate == machine.lights {
                    //huh there is a count ones function on integer, neat
                    return newvisit.count_ones() as usize;
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
