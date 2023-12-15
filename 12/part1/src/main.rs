use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn check_line(line: &[char], damaged: &[u32]) -> u32 {
    let string: String = line.iter().collect();
    let chunk_len = string
        .split('.')
        .filter(|chunk| !chunk.is_empty())
        .map(|chunk| chunk.len() as u32)
        .collect::<Vec<u32>>();

    if chunk_len == damaged {
        1
    } else {
        0
    }
}

fn solve_line(line: &mut Vec<char>, damaged: &[u32]) -> u32 {
    let unknown = line.iter().by_ref().position(|&x| x == '?');
    if unknown.is_none() {
        return check_line(line, damaged);
    }

    let unknown = unknown.unwrap();
    line[unknown] = '#';
    let combination = solve_line(line, damaged);

    line[unknown] = '?';
    combination
}

fn main() {
    let file = File::open("test.txt").unwrap();
    let reader = BufReader::new(&file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();
    let lines: Vec<(&str, Vec<u32>)> = lines
        .iter()
        .map(|s| {
            let split = s.split(' ').collect::<Vec<&str>>();
            (
                split[0],
                split[1]
                    .split(',')
                    .filter_map(|c| c.parse().ok())
                    .collect::<Vec<u32>>(),
            )
        })
        .collect();

    let mut res = 0;
    for (line, damaged_springs) in lines {
        let mut line = line.chars().collect::<Vec<char>>();
        res += solve_line(&mut line, &damaged_springs);
    }
    println!("The sum of all counts is {}", res);
}
