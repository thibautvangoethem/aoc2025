use std::{
    collections::{HashMap, VecDeque},
    fs,
    str::FromStr,
};
//you have no clue how long it took to install this on windows ...
use z3::{self, Optimize, SatResult};

pub fn solve(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    solve1(path)?;
    solve2(path)?;
    Ok(())
}

#[derive(Debug, Clone, PartialEq)]
struct BitMachine {
    lights: u16,
    button_wiring: Vec<u16>,
    joltage_req: Vec<u16>,
}

#[derive(Clone, Debug)]
struct ParseError;
impl FromStr for BitMachine {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, ParseError> {
        let vals: Vec<&str> = s.trim().split(" ").collect();
        let mut lights: u16 = 0;
        let mut wiring: Vec<u16> = Vec::new();
        let mut jolt: Vec<u16> = Vec::new();
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

        Ok(BitMachine {
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

fn bfs_search_machine(machine: &BitMachine) -> usize {
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
        .map(|line| BitMachine::from_str(line).unwrap())
        .collect::<Vec<BitMachine>>();
    println!(
        "the result for day10:1 is {0:?}",
        race_against
            .iter()
            .fold(0, |acc, machine| acc + bfs_search_machine(machine))
    );
    Ok(())
}

//This goes in the museum of bad ideas, brute force is not the way
// #[derive(Clone, Debug)]
// struct VisitingJolted {
//     state: u16,
//     jolts: Vec<u16>,
// }

// fn bfs_search_machine_jolted(machine: &BitMachine) -> usize {
//     let mut searching: VecDeque<VisitingJolted> = VecDeque::from([VisitingJolted {
//         state: 0,
//         jolts: vec![0; machine.joltage_req.len()],
//     }]);
//     while let Some(handling) = searching.pop_front() {
//         'nextwire: for val in machine.button_wiring.iter() {
//             let mut newjolt = handling.jolts.clone();
//             // for i in newjolt.iter() {
//             //     print!("{},", i);
//             // }
//             // println!("");
//             let mut idx: usize = val.trailing_zeros() as usize;
//             let mut val = val.clone();
//             while idx < 16 {
//                 newjolt[idx] += 1;
//                 if newjolt[idx] > machine.joltage_req[idx] {
//                     continue 'nextwire;
//                 }
//                 val &= !(1 << idx);
//                 idx = val.trailing_zeros() as usize;
//             }
//             let newstate = handling.state ^ val;
//             if newstate == machine.lights && newjolt == machine.joltage_req {
//                 //todo actually count, want to see it finish first
//                 return 1;
//             }

//             searching.push_back(VisitingJolted {
//                 state: newstate,
//                 jolts: newjolt,
//             });
//         }
//     }
//     panic!("it should have finished at this points");
// }
#[derive(Debug, Clone, PartialEq)]
struct Machine {
    //no longer relevant here
    lights: Vec<bool>,
    button_wiring: Vec<Vec<i32>>,
    joltage_req: Vec<i32>,
}
impl FromStr for Machine {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, ParseError> {
        let vals: Vec<&str> = s.trim().split(" ").collect();
        let mut lights: Vec<bool> = Vec::new();
        let mut wiring: Vec<Vec<i32>> = Vec::new();
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
                let mut subwiring: Vec<i32> = vec![];
                while let Some(val) = c_iter.next() {
                    if val != ',' && val != ')' {
                        let i = val.to_digit(10).unwrap();
                        subwiring.push(i as i32);
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

// fn get_max_presses(joltage_left: &Vec<i32>, press_wiring: &Vec<i32>) -> i32 {
//     //Is this oneliner enough?
//     joltage_left[*press_wiring
//         .iter()
//         .min_by(|a, b| joltage_left[**a as usize].cmp(&joltage_left[**b as usize]))
//         .unwrap() as usize]
// }

// fn get_highest_req_idx(joltage_left: &Vec<i32>) -> usize {
//     joltage_left
//         .iter()
//         .enumerate()
//         .max_by(|(_, a), (_, b)| a.cmp(b))
//         .map(|(index, _)| index)
//         .unwrap()
// }

// fn is_all_zero(joltage_left: &Vec<i32>) -> bool {
//     joltage_left.iter().sum::<i32>() == 0
// }

// fn print_joltage(joltage_left: &Vec<i64>) {
//     for i in joltage_left {
//         print!("{},", i);
//     }
//     println!();
// }

// fn init_fill<'a>(
//     joltage_left: &mut Vec<i32>,
//     press_counter: &mut HashMap<&'a Vec<i32>, i32>,
//     button_to_wiring: &HashMap<i32, Vec<&'a Vec<i32>>>,
// ) {
//     let mut previdx = 100;
//     let mut idx = 101;
//     'cont: while previdx != idx {
//         // print_joltage(&joltage_left);
//         idx = get_highest_req_idx(&joltage_left);
//         //sort our button_to_wiring by importance
//         let val = button_to_wiring.get(&(idx as i32)).unwrap();

//         for try_apply in val.iter() {
//             let max = get_max_presses(&joltage_left, *try_apply);
//             // print_joltage(&try_apply);
//             // println!("{}", max);
//             if max == 0 {
//                 continue;
//             } else {
//                 previdx = 101;
//                 *press_counter.get_mut(try_apply).unwrap() += max;
//                 for i in try_apply.iter() {
//                     joltage_left[*i as usize] -= max;
//                 }
//                 continue 'cont;
//                 // retry and increment new joltage
//             }
//         }
//         previdx = idx.clone();
//     }
// }
// //After lots of considerations and being stuck with this custom solution, I am giving up. Time for a boring matrix linear solver :(
// fn solve_joltage(machine: &Machine) -> i32 {
//     //Alright idea time. I see that it generally is always smarter to choose wiring with more presses at the start.
//     // So I make a hashmap that basically goes 1 -> list of voltages that increase 1 Sorted from most to least presses at once
//     // then for every lamp I write an algorithm that says: for every lamp, get the highest presses at once that would prevent us from going over
//     // So if the req is 4 2, and we start at 0 0 with wires (0,1) and (0), then we first find presses for 0 => highest is (0,1) press that, same again press that again
//     // Now we are at 2,2, presses the 0,1 again would put is over, so we choose the next (0) and press that => twice.
//     // Problem if we have req 4 2 3 with wires (0,1) (0,2) and (1), algorithm would first choose 0,1 for twice 2 2 0 => then 0,2 twice => 4 2 2, but now there is nothing to satisfy the third value
//     // => small optimization, rank based on req too?  => nvm this wouldnt work
//     // In this case an algorithm is needed that can unpress (brute force) => fulfill a number by always unpressing the smallest presses first, and try replacing them with the highest presses filling it    // Do this untill the conflicting number is filled, then try the previous algorithm before of filling in the number with highest req

//     //Setup
//     let mut press_counter: HashMap<&Vec<i32>, i32> =
//         machine.button_wiring.iter().map(|val| (val, 0)).collect();
//     let mut button_to_wiring: HashMap<i32, Vec<&Vec<i32>>> = HashMap::new();
//     for wiring in machine.button_wiring.iter() {
//         for i in wiring {
//             if !button_to_wiring.contains_key(&i) {
//                 button_to_wiring.insert(*i, Vec::new());
//             }
//             button_to_wiring.get_mut(i).unwrap().push(wiring);
//         }
//     }
//     for (_, val) in button_to_wiring.iter_mut() {
//         val.sort_by(|a, b| b.len().cmp(&a.len()));
//     }
//     // // Idea is to have light idx => options to unpress
//     // // an options is here a wiring to list of buttons to unpress to allow this wiring to be pressed
//     // // so idx=> map (vec1 => vec2<vec3<vec4>>)
//     // // vec1 is wiring you want to try and press
//     // // vec2 is grouping of an options of unpresses
//     // // vec3 is ab option grouping of unpresses to do for the unpress
//     // // vec4 is wiring to be unpressed
//     // // so if you ahve wirign

//     // let mut button_to_unpressers: HashMap<i32, Vec<&Vec<i32>>> = HashMap::new();
//     // for (idx, wiring) in button_to_wiring.iter() {
//     //     for i in wiring.iter() {
//     //         for j in i.iter() {
//     //             if *j == *idx {
//     //                 continue;
//     //             }
//     //             if !button_to_unpressers.contains_key(j) {
//     //                 button_to_unpressers.insert(*j, Vec::new());
//     //             }
//     //             if button_to_unpressers.get(j).unwrap().contains(&i) {
//     //                 continue;
//     //             }
//     //             button_to_unpressers.get_mut(j).unwrap().push(i);
//     //         }
//     //     }
//     // }
//     // for (_, vals) in button_to_unpressers.iter_mut() {
//     //     //smallest first here always
//     //     vals.sort_by(|a, b| a.len().cmp(&b.len()));
//     // }
//     let mut joltage_left = machine.joltage_req.clone();
//     init_fill(&mut joltage_left, &mut press_counter, &button_to_wiring);
//     //algorithm goes here

//     // here goes the go back thingy
//     // let unpress_options: Vec<&Vec<i32>> = button_to_unpressers
//     //     .get(&(idx as i32))
//     //     .unwrap()
//     //     .iter()
//     //     .filter(|&&option| *press_counter.get(option).unwrap() > 0)
//     //     .map(|press| *press)
//     //     .collect();
//     // if unpress_options.is_empty() {
//     //     panic!("no options to unpress found")
//     // }
//     // for unpress in unpress_options {
//     //     *press_counter.get_mut(unpress).unwrap() -= 1;
//     //     for i in unpress.iter() {
//     //         joltage_left[*i as usize] += 1;
//     //     }
//     // }

//     press_counter.iter().map(|(_, val)| val).sum()
// }
fn z3_solve_joltage(machine: &Machine) -> i32 {
    //idea solver where for every wiring we make a unique int
    // Then for every light that wiring would turn on we make an assert such that int1 + int2+int3 ... = total for that light
    let mut button_to_wiring: HashMap<i32, Vec<&Vec<i32>>> = HashMap::new();
    for wiring in machine.button_wiring.iter() {
        for i in wiring {
            if !button_to_wiring.contains_key(&i) {
                button_to_wiring.insert(*i, Vec::new());
            }
            button_to_wiring.get_mut(i).unwrap().push(wiring);
        }
    }
    // let solver = Solver::new();
    let solver = Optimize::new();
    let mut wiring_to_int: HashMap<&Vec<i32>, z3::ast::Int> = HashMap::new();
    for wiring in machine.button_wiring.iter() {
        wiring_to_int.insert(wiring, z3::ast::Int::fresh_const("NULL"));
    }
    for (_vec, var) in wiring_to_int.iter() {
        solver.assert(&var.ge(0));
    }

    for (index, vecs) in button_to_wiring.iter() {
        let sumation = vecs
            .iter()
            .map(|val| wiring_to_int.get(val).unwrap())
            .fold(z3::ast::Int::from_i64(0), |val, v| val + v);
        let val = machine.joltage_req.get(*index as usize).unwrap().to_owned();
        solver.assert(&sumation.eq(val));
    }

    let sum = wiring_to_int
        .values()
        .fold(z3::ast::Int::from_i64(0), |val, v| val + v);
    solver.minimize(&sum);

    let result = solver.check(&Vec::new());
    if result == SatResult::Sat {
        let model = solver.get_model().unwrap();
        let result: i64 = wiring_to_int
            .values()
            .into_iter()
            .map(|val| model.get_const_interp(val).unwrap().as_i64().unwrap())
            .sum();
        //downcast viable, result should never go beyond this
        return result as i32;
    }
    panic!("the world exploded, division by zero occured or something");
}

pub fn solve2(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let binding = fs::read_to_string(path).expect("Should have been able to read the file");
    let race_against = binding
        .lines()
        .map(|line| Machine::from_str(line).unwrap())
        .collect::<Vec<Machine>>();
    println!(
        "the result for day10:2 is {0:?}",
        race_against
            .iter()
            .fold(0, |acc, machine| acc + z3_solve_joltage(machine))
    );
    Ok(())
}
