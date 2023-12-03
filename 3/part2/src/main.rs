fn check_valid(
    x: usize,
    y: usize,
    table: &Vec<Vec<char>>,
) -> Vec<(usize, usize)> {
    // check in 8 positions
    let mut truth_table: Vec<(usize, usize)> = Vec::new();

    //upper left
    if table[y + 1][x - 1].is_ascii_digit() {
        truth_table.push((y + 1, x - 1));
    }
    //up
    if table[y + 1][x].is_ascii_digit() {
        truth_table.push((y + 1, x));
    }
    //upper right
    if table[y + 1][x + 1].is_ascii_digit() {
        truth_table.push((y + 1, x + 1));
    }
    //left
    if table[y][x - 1].is_ascii_digit() {
        truth_table.push((y, x - 1));
    }
    //right
    if table[y][x + 1].is_ascii_digit() {
        truth_table.push((y, x + 1));
    }

    //down left
    if table[y - 1][x - 1].is_ascii_digit() {
        truth_table.push((y - 1, x - 1));
    }
    //down
    if table[y - 1][x].is_ascii_digit() {
        truth_table.push((y - 1, x));
    }

    //down right
    if table[y - 1][x + 1].is_ascii_digit() {
        truth_table.push((y - 1, x + 1));
    }

    truth_table
}

fn build_num(mut x: usize, y: usize, table: &Vec<Vec<char>>) -> u32 {
    let mut res = 0;
    let mut offset = 0;
    while table[y][x - offset].is_ascii_digit() {
        offset += 1;
    }

    offset -= 1;
    x -= offset;
    offset = 0;

    while table[y][x + offset].is_ascii_digit() {
        res *= 10;
        res += table[y][x + offset].to_digit(10).unwrap();
        offset += 1;
    }

    res
}

fn clean_gear(gear_table: &mut Vec<(usize, usize)>) {
    let mut i = 0;
    while i < gear_table.len() - 1 {
        if gear_table[i].0 == gear_table[i + 1].0
            && gear_table[i].1.abs_diff(gear_table[i + 1].1) == 1
        {
            gear_table.remove(i);
            dbg!(&gear_table);
        } else {
            i += 1;
        }
    }
}

fn multiply_gear(
    gear_table: &mut Vec<(usize, usize)>,
    table: &Vec<Vec<char>>,
) -> u32 {
    clean_gear(gear_table);
    if gear_table.len() != 2 {
        return 0;
    }

    let first = gear_table.first().unwrap();
    let second = gear_table.last().unwrap();
    let first_num = build_num(first.1, first.0, table);
    let second_num = build_num(second.1, second.0, table);
    println!(
        "{} * {} = {}",
        first_num,
        second_num,
        first_num * second_num
    );
    first_num * second_num
}

fn main() {
    let mut res = 0;
    let mut table: Vec<Vec<char>> = Vec::new();
    let binding = std::fs::read_to_string("test.txt").unwrap();

    for line in binding.lines() {
        let mut chars: Vec<char> = line.chars().collect();
        chars.insert(0, '.');
        chars.push('.');
        table.push(chars);
    }

    let mut padding: Vec<char> = Vec::new();

    for _ in 0..table[0].len() {
        padding.push('.');
    }

    table.insert(0, padding.clone());
    table.push(padding);

    for y in 1..table.len() - 1 {
        let mut x = 1;
        while x < table[y].len() - 1 {
            if table[y][x] == '*' {
                let mut num_positions = check_valid(x, y, &table);
                res += multiply_gear(&mut num_positions, &table);
            }
            x += 1;
        }
    }

    println!("result is {}", res);
}
