use std::fs;

pub fn solve(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    solve1(path)?;
    solve2(path)?;
    Ok(())
}

pub fn solve1(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let binding = fs::read_to_string(path).expect("Should have been able to read the file");
    let contents = binding.lines();

    let mut numbers: Vec<Vec<u64>> = Vec::new();
    let mut operators: Vec<char> = Vec::new();

    for line in contents {
        for (idx, word) in line.split_whitespace().enumerate() {
            if numbers.len() <= idx {
                numbers.push(Vec::new());
            }

            let value = word.parse::<u64>();
            match value {
                Ok(val) => numbers[idx].push(val),
                Err(_) => operators.push(word.chars().next().unwrap()),
            }
        }
    }

    let mut total: u64 = 0;
    for (idx, numberset) in numbers.iter().enumerate() {
        let mut subcount: u64 = match operators[idx] {
            '*' => 1,
            '+' => 0,
            _ => panic!("unsupported operator"),
        };

        for number in numberset {
            match operators[idx] {
                '*' => subcount *= number,
                '+' => subcount += number,
                _ => panic!("unsupported operator"),
            };
        }
        total += subcount;
    }
    println!("the result for day06:1 is {0:?}", total);
    Ok(())
}
pub fn solve2(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let binding = fs::read_to_string(path).expect("Should have been able to read the file");
    let mut contents = binding.lines().peekable();

    let mut numbers_char: Vec<Vec<char>> = Vec::new();
    let mut operators: Vec<char> = Vec::new();

    while let Some(line) = contents.next() {
        //case operator row
        if contents.peek().is_none() {
            for word in line.split_whitespace() {
                operators.push(word.chars().next().unwrap());
            }
            continue;
        }
        numbers_char.push(line.chars().collect());
    }

    let mut numbers: Vec<Vec<u64>> = Vec::new();
    let mut counter = 0;
    for innerindex in 0..numbers_char[0].len() {
        let mut concating: String = "".to_string();
        for numbercharvec in &numbers_char {
            if numbercharvec[innerindex] != ' ' {
                concating.push(numbercharvec[innerindex]);
            }
        }

        if concating.is_empty() {
            counter += 1
        } else {
            if numbers.len() <= counter {
                numbers.push(Vec::new());
            }
            numbers[counter].push(concating.parse()?);
        }
    }
    let mut total: u64 = 0;
    for (idx, numberset) in numbers.iter().enumerate() {
        let mut subcount: u64 = match operators[idx] {
            '*' => 1,
            '+' => 0,
            _ => panic!("unsupported operator"),
        };

        for number in numberset {
            match operators[idx] {
                '*' => subcount *= number,
                '+' => subcount += number,
                _ => panic!("unsupported operator"),
            };
        }
        total += subcount;
    }
    println!("the result for day06:2 is {0:?}", total);
    Ok(())
}
