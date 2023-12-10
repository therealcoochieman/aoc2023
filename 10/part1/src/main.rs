use std::{
    fs::File,
    io::{BufRead, BufReader},
};

type Coordinates = (usize, usize);

#[derive(PartialEq, Eq, Debug)]
enum Direction {
    Down,
    Left,
    Right,
    Up,
    None,
}

impl Direction {
    fn go_from(&self, coord: Coordinates) -> Coordinates {
        match self {
            Direction::Down => (coord.0, coord.1 + 1),
            Direction::Left => (coord.0 - 1, coord.1),
            Direction::Right => (coord.0 + 1, coord.1),
            Direction::Up => (coord.0, coord.1 - 1),
            Direction::None => coord, // cant go anywhere
        }
    }

    fn inverse(&self) -> Self {
        match self {
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
            Direction::None => unreachable!(),
        }
    }
}

type PipeType = Vec<Direction>;

#[derive(Debug)]
struct Pipe {
    pub coord: Coordinates,
    pub r#type: PipeType,
}

impl Default for Pipe {
    fn default() -> Self {
        Pipe::new('.', (0, 0))
    }
}

impl Pipe {
    pub fn new(char: char, coord: Coordinates) -> Self {
        let r#type = match char {
            '|' => vec![Direction::Up, Direction::Down],
            '-' => vec![Direction::Left, Direction::Right],
            'L' => vec![Direction::Up, Direction::Right],
            'J' => vec![Direction::Up, Direction::Left],
            '7' => vec![Direction::Down, Direction::Left],
            'F' => vec![Direction::Down, Direction::Right],
            '.' => vec![Direction::None, Direction::None],
            'S' => vec![
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ],
            _ => unreachable!(),
        };

        Self { coord, r#type }
    }
}

fn build_padding_pipe_line(len: usize) -> Vec<Pipe> {
    let mut padding = Vec::new();
    for _ in 0..len {
        padding.push(Pipe::default());
    }

    padding
}
fn create_pipes(lines: &[String]) -> (Vec<Vec<Pipe>>, Coordinates) {
    let mut start_pipe = None;
    let mut pipes = Vec::new();
    for (y, line) in lines.iter().enumerate() {
        let mut pipe_line = Vec::new();
        pipe_line.push(Pipe::default()); // left padding

        for (x, char) in line.chars().enumerate() {
            pipe_line.push(Pipe::new(char, (x + 1, y + 1)));
            if char == 'S' {
                start_pipe = Some((x + 1, y + 1));
            }
        }

        pipe_line.push(Pipe::default()); // right padding
        pipes.push(pipe_line);
    }

    pipes.insert(0, build_padding_pipe_line(pipes[0].len()));
    pipes.push(build_padding_pipe_line(pipes[0].len()));

    (pipes, start_pipe.expect("No start type??"))
}

// fn pipe_dfs(
//     start: &Pipe,
//     pipes: &[Vec<Pipe>],
//     marked: &mut Vec<Coordinates>,
// ) -> bool {
//     marked.push(start.coord);
//
//     let first_direction = start.r#type[0].go_from(start.coord);
//     let first_pipe = pipes[first_direction.1][first_direction.0];
//     let mut first_loop = false;
//     if first_pipe.r#type.contains(&start.r#type[0].inverse())
//         && !marked.contains(&first_pipe.coord)
//     {
//         first_loop = pipe_dfs(&first_pipe, pipes, marked);
//     }
//
//     let second_direction = start.r#type[1].go_from(start.coord);
//     let second_pipe = pipes[second_direction.1][second_direction.0];
//     let mut second_loop = false;
//     if second_pipe.r#type.contains(&start.r#type[0].inverse())
//         && !marked.contains(&second_pipe.coord)
//     {
//         second_loop = pipe_dfs(&second_pipe, pipes, marked);
//     }
//
//     second_loop || first_loop
// }

fn find_loop(
    start: Coordinates,
    pipes: &[Vec<Pipe>],
    visited: &mut Vec<Coordinates>,
    rec_stack: &mut Vec<(Coordinates, usize)>,
    step: usize,
) -> bool {
    let pipe = &pipes[start.1][start.0];

    if pipe.r#type.len() > 2 {
        return true;
    }

    visited.push(start);
    rec_stack.push((start, step));
    for direction in &pipe.r#type {
        let direction_coord = direction.go_from(start);
        let direction_pipe = &pipes[direction_coord.1][direction_coord.0];

        if !direction_pipe.r#type.contains(&Direction::None)
            && direction_pipe.r#type.contains(&direction.inverse())
            && !visited.contains(&direction_coord)
        {
            if find_loop(direction_coord, pipes, visited, rec_stack, step + 1) {
                return true;
            }
        }
    }

    rec_stack.pop();
    false
}

fn get_res(max: usize) -> usize {
    if max % 2 == 0 {
        max / 2
    } else {
        (max + 1) / 2
    }
}
fn find_loop_main(
    start: Coordinates,
    pipes: &mut [Vec<Pipe>],
) -> (Vec<Coordinates>, usize) {
    let (mut visited, mut rec_stack) = (Vec::new(), Vec::new());
    let look_vec = vec![
        (Direction::Up, Direction::Up.go_from(start)),
        (Direction::Down, Direction::Down.go_from(start)),
        (Direction::Left, Direction::Left.go_from(start)),
        (Direction::Right, Direction::Right.go_from(start)),
    ];

    for (val, direction) in look_vec {
        if !visited.contains(&direction)
            && !pipes[direction.1][direction.0]
                .r#type
                .contains(&Direction::None)
        {
            let index = pipes[start.1][start.0]
                .r#type
                .iter()
                .position(|x| *x == val)
                .unwrap();

            pipes[start.1][start.0].r#type.remove(index);

            if find_loop(direction, pipes, &mut visited, &mut rec_stack, 1) {
                let max =
                    rec_stack.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();
                let path = rec_stack.iter().map(|step| step.0).collect();
                println!("res is {}", get_res(max.1));
                return (path, get_res(max.1));
            }

            pipes[start.1][start.0].r#type.push(val);
        }
    }

    (Vec::new(), 0)
}
fn main() {
    let file = File::open("test.txt").unwrap();
    let reader = BufReader::new(&file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();
    let (mut pipes, start_pipe) = create_pipes(&lines);
    let (path, res) =  find_loop_main(start_pipe, &mut pipes);
    println!("haha {}", res);
}
