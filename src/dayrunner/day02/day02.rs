use fancy_regex::Regex;
use std::{fs, str::FromStr};
// brute force solution, only had 1 hour today, not going to go all smart on this
//not i32 because i dont know yet whether the ranges will be within 2^32
// => checked u64 should be fine
struct Range {
    start: u64,
    end: u64,
}
struct ParseError;
impl FromStr for Range {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, ParseError> {
        let vals: Vec<&str> = s.trim().split("-").collect();
        assert!(vals.len() == 2);
        let mut iter = vals.iter();
        let val1 = iter.next().to_owned().unwrap();
        let val2 = iter.next().to_owned().unwrap();
        Ok(Range {
            start: val1.parse().unwrap(),
            end: val2.parse().unwrap(),
        })
    }
}
pub fn solve(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    solve1(path)?;
    solve2(path)?;
    Ok(())
}

pub fn solve1(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let binding = fs::read_to_string(path).expect("Should have been able to read the file");
    let contents = binding.split(",");
    let mut funky_ids: Vec<u64> = Vec::new();
    let re: Regex = Regex::new(r"^(\d+)\1$").unwrap();
    for line in contents {
        let parsed_line = match Range::from_str(line) {
            Ok(val) => val,
            Err(_error) => panic!("Could not parse: {line:?}"),
        };
        for n in parsed_line.start..parsed_line.end + 1 {
            if re.is_match(n.to_string().as_str()).unwrap() {
                // println!("{}", n);
                funky_ids.push(n);
            }
        }
    }
    let result: u64 = funky_ids.iter().sum();
    println!("the result for day02:1 is {0:?}", result);
    Ok(())
}
pub fn solve2(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let binding = fs::read_to_string(path).expect("Should have been able to read the file");
    let contents = binding.split(",");
    let mut funky_ids: Vec<u64> = Vec::new();
    let re: Regex = Regex::new(r"^(\d+)(\1)+$").unwrap();
    for line in contents {
        let parsed_line = match Range::from_str(line) {
            Ok(val) => val,
            Err(_error) => panic!("Could not parse: {line:?}"),
        };
        for n in parsed_line.start..parsed_line.end + 1 {
            if re.is_match(n.to_string().as_str()).unwrap() {
                funky_ids.push(n);
            }
        }
    }
    let result: u64 = funky_ids.iter().sum();
    println!("the result for day02:2 is {0:?}", result);
    Ok(())
}
