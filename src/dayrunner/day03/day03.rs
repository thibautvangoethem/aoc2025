use itertools::peek_nth;
use std::fs;
pub fn solve(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    solve1(path)?;
    solve2(path)?;
    Ok(())
}

pub fn solve1(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let binding = fs::read_to_string(path).expect("Should have been able to read the file");
    let contents = binding.lines();
    let mut total = 0;
    for line in contents {
        let mut valfirst = 0;
        let mut valsecond = 0;
        let mut iter = line.chars().peekable();
        while let Some(char) = iter.next() {
            let digit = char.to_digit(10).unwrap();
            //new first index, clear all previous, unless last idx
            if digit > valfirst && !iter.peek().is_none() {
                valfirst = digit;
                valsecond = 0;
            } else if digit > valsecond {
                valsecond = digit;
            }
        }
        // println!("{} {} {} {}", valfirst, firstidx, valsecond, secondidx);
        total += valfirst * 10 + valsecond;
    }

    println!("the result for day03:1 is {0:?}", total);
    Ok(())
}

pub fn solve2(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let binding = fs::read_to_string(path).expect("Should have been able to read the file");
    let contents = binding.lines();
    let mut total: u64 = 0;
    for line in contents {
        let mut vals: [u32; 12] = [0; 12];

        let mut iter: itertools::PeekNth<std::str::Chars<'_>> = peek_nth(line.chars());
        while let Some(char) = iter.next() {
            let digit = char.to_digit(10).unwrap();
            let valiter = vals.iter_mut().enumerate();
            let mut clearing = false;
            for (idx, val) in valiter {
                if clearing {
                    *val = 0;
                    continue;
                }
                let condition = if idx < 11 {
                    //Apparently peek_nth(0) is basically peek(), I thought it would get the current char, that wasted some time ...
                    iter.peek_nth(10 - idx).is_none()
                } else {
                    false
                };
                if digit > *val && !condition {
                    *val = digit;
                    clearing = true;
                }
            }
        }
        let val = vals
            .iter()
            .fold(0, |acc: u64, elem| acc * 10 + u64::from(*elem));
        // println!("{}", val);
        total += val;
    }
    println!("the result for day03:2 is {0:?}", total);
    Ok(())
}
