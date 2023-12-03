fn check_valid(x: usize, y: usize, table: &Vec<Vec<char>>) -> bool {
    // check in 8 positions
    if !table[y][x].is_ascii_digit() {
        return false;
    }

    //upper left
    if !table[y + 1][x - 1].is_ascii_digit() && table[y + 1][x - 1] != '.' {
        return true;
    }
    //up
    if !table[y + 1][x].is_ascii_digit() && table[y + 1][x] != '.' {
        return true;
    }
    //upper right
    if !table[y + 1][x + 1].is_ascii_digit() && table[y + 1][x + 1] != '.' {
        return true;
    }
    //left
    if !table[y][x - 1].is_ascii_digit() && table[y][x - 1] != '.' {
        return true;
    }
    //right
    if !table[y][x + 1].is_ascii_digit() && table[y][x + 1] != '.' {
        return true;
    }

    //down left
    if !table[y - 1][x - 1].is_ascii_digit() && table[y - 1][x - 1] != '.' {
        return true;
    }
    //down
    if !table[y - 1][x].is_ascii_digit() && table[y - 1][x] != '.' {
        return true;
    }
    //down right
    if !table[y - 1][x + 1].is_ascii_digit() && table[y - 1][x + 1] != '.' {
        return true;
    }

    //if right is number then go check that way
    if table[y][x + 1].is_ascii_digit() {
        return check_valid(x + 1, y, table);
    }

    false
}
fn build_num(x: usize, y: usize, table: &Vec<Vec<char>>) -> (u32, u32) {
    let mut res = 0;
    let mut offset = 0;
    while table[y][x + offset].is_ascii_digit() {
        res *= 10;
        res += table[y][x + offset].to_digit(10).unwrap();
        offset += 1;
    }

    (res, offset as u32)
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
            if check_valid(x, y, &table) {
                let (num, offset) = build_num(x, y, &table);
                res += num;
                x += offset as usize;
            } else {
                x += 1;
            }
        }
    }

    println!("result is {}", res);
}
