use std::{
    fs::File,
    io::{BufRead, BufReader},
};

type Coordinates = (u64, u64);
const EXPANSION: u64 = 1_000_000;

fn manhattan_galaxy(galaxies: Vec<Coordinates>) -> u64 {
    let mut thing = 0;
    for i in 0..galaxies.len() - 1 {
        let source = galaxies[i];
        for distance in galaxies.iter().skip(i + 1) {
            let manhattan =
                source.0.abs_diff(distance.0) + source.1.abs_diff(distance.1);

            thing += manhattan;
        }
    }

    thing
}

#[derive(Debug)]
struct Expansion {
    pub expansion: Vec<(usize, u64)>,
}

impl Expansion {
    pub fn compute_index(&self, index: usize) -> u64 {
        let mut closest_expansion = None;
        for exp in &self.expansion {
            if index >= exp.0 {
                closest_expansion = Some(exp);
            } else {
                break;
            }
        }

        let closest_expansion = closest_expansion.unwrap_or(&(0, 0));

        closest_expansion.0.abs_diff(index) as u64
            + closest_expansion.0 as u64
            + closest_expansion.1
    }

    pub fn add_expansion(&mut self, index: usize, expansion_value: u64) {
        self.expansion.push((index, expansion_value));
        self.expansion.sort_by(|a, b| a.0.cmp(&b.0));
        let position = self
            .expansion
            .iter()
            .by_ref()
            .position(|&exp| exp == (index, expansion_value))
            .unwrap();

        if position == self.expansion.len() - 1 && self.expansion.len() >= 2 {
            self.expansion[position].1 +=
                self.expansion[self.expansion.len() - 2].1;
        }
    }
}
fn expand_space(lines: &mut Vec<Vec<char>>) -> Vec<Coordinates> {
    let mut galaxies = Vec::new();

    let mut x_expansion = Expansion {
        expansion: Vec::new(),
    };
    let mut y_expansion = Expansion {
        expansion: Vec::new(),
    };

    let mut x = 0;
    let x_len = lines[0].len();
    while x < x_len {
        let mut galaxy_count = 0;
        for line_y in &*lines {
            if line_y[x] == '#' {
                galaxy_count += 1;
            }
        }

        if galaxy_count == 0 {
            x_expansion.add_expansion(x, EXPANSION - 1);
        }

        x += 1;
    }

    let mut y = 0;
    let y_len = lines.len();
    while y < y_len {
        let thing = lines[y].iter().filter(|&c| c == &'#').count();

        if thing == 0 {
            y_expansion.add_expansion(y, EXPANSION - 1);
        }

        y += 1;
    }

    for (y, line) in lines.iter().enumerate() {
        for (x, &c) in line.iter().enumerate() {
            if c == '#' {
                let galaxy = (
                    x_expansion.compute_index(x),
                    y_expansion.compute_index(y),
                );
                galaxies.push(galaxy);
            }
        }
    }

    galaxies
}

fn main() {
    let file = File::open("test.txt").unwrap();
    let reader = BufReader::new(&file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();
    let mut lines = lines
        .iter()
        .map(|s| s.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let galaxies = expand_space(&mut lines);

    println!("the result is {}", manhattan_galaxy(galaxies))
}
