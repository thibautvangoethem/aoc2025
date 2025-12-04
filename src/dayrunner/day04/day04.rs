use std::fs;

pub fn solve(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    solve1(path)?;
    solve2(path)?;
    Ok(())
}

fn get_accessible_indexes(matrix: &Vec<Vec<char>>) -> Vec<[usize; 2]> {
    let mut result: Vec<[usize; 2]> = Vec::new();

    for i in 1..matrix.len() - 1 {
        for j in 1..matrix.len() - 1 {
            if matrix[i][j] != '@' {
                // print!(" ");
                continue;
            }
            let mut subcounter = 0;
            for ix in i - 1..i + 2 {
                for jx in j - 1..j + 2 {
                    if ix == i && jx == j {
                        continue;
                    }
                    if matrix[ix][jx] == '@' {
                        subcounter += 1
                    }
                }
            }
            // print!("{}", subcounter);
            if subcounter < 4 {
                result.push([i, j]);
            }
        }
        // println!("");
    }
    result
}

pub fn solve1(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let binding = fs::read_to_string(path).expect("Should have been able to read the file");
    let contents = binding.lines();
    let mut thegrid: Vec<Vec<char>> = Vec::new();

    //apparently it is extremely difficult to get the subelement size
    let mut firstbool = true;
    let mut size = 0;
    for line in contents {
        if firstbool {
            size = line.chars().count();
            thegrid.push(vec!['.'; size + 2]);
            firstbool = false;
        }
        let mut line: Vec<char> = line.chars().collect();
        line.insert(0, '.');
        line.push('.');
        thegrid.push(line);
    }
    thegrid.push(vec!['.'; size + 2]);
    let resultcounter = get_accessible_indexes(&thegrid).len();

    println!("the result for day04:1 is {0:?}", resultcounter);
    Ok(())
}

pub fn solve2(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let binding = fs::read_to_string(path).expect("Should have been able to read the file");
    let contents = binding.lines();
    let mut thegrid: Vec<Vec<char>> = Vec::new();

    //apparently it is extremely difficult to get the subelement size
    let mut firstbool = true;
    let mut size = 0;
    for line in contents {
        if firstbool {
            size = line.chars().count();
            thegrid.push(vec!['.'; size + 2]);
            firstbool = false;
        }
        let mut line: Vec<char> = line.chars().collect();
        line.insert(0, '.');
        line.push('.');
        thegrid.push(line);
    }
    thegrid.push(vec!['.'; size + 2]);
    //at this point the grid is made
    let mut resultcounter = 0;
    loop {
        let indeces = get_accessible_indexes(&thegrid);
        if indeces.is_empty() {
            break;
        }
        resultcounter += indeces.len();
        for idx in indeces {
            thegrid[idx[0]][idx[1]] = 'x'
        }
    }
    println!("the result for day04:2 is {0:?}", resultcounter);
    Ok(())
}
