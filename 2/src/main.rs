use std::cmp::max;

#[derive(Debug)]
struct Game {
    pub num: u32,
    pub green: u32,
    pub red: u32,
    pub blue: u32,
}

fn line_to_game(line: &str, num: u32) -> Game {
    let line = &line.split(':').collect::<Vec<&str>>()[1];
    let rounds: Vec<&str> = line.split(';').collect();
    let (mut green, mut red, mut blue) = (0, 0, 0);
    for round in rounds {
        let colors: Vec<&str> = round.split(',').map(|s| s.trim()).collect();
        for color in colors {
            let tokens: Vec<&str> = color.split(' ').map(|s| s.trim()).collect();
            let color_number: u32 = tokens[0].parse().unwrap();
            match tokens[1] {
                "green" => green = max(color_number, green),
                "red" => red = max(color_number, red),
                "blue" => blue = max(color_number, blue),
                e => eprintln!("{} unknown color", e),
            }
        }
    }

    Game {
        num,
        green,
        red,
        blue,
    }
}

fn main() {
    let mut res = 0;
    for (index, line) in std::fs::read_to_string("test.txt")
        .unwrap()
        .lines()
        .enumerate()
    {
        let game = line_to_game(line, index as u32 + 1);
        res += game.red * game.blue * game.green;
    }
    println!("result is {}", res);
}
