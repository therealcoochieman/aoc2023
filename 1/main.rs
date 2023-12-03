static DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn fill_with_num(line_numbers: &mut Vec<(usize, char)>, line: &str) {
    for (i, digit) in DIGITS.iter().enumerate() {
        let mut line_copy = line;
        let mut index = 0;
        loop {
            let number = line_copy.find(digit);
            if number.is_none() {
                break;
            }

            let start_index = number.unwrap();
            line_numbers.push((
                start_index + index,
                char::from_digit((i + 1) as u32, 10).unwrap(),
            ));

            line_copy = &line_copy[start_index + digit.len()..];
	    index += start_index + digit.len();
        }
    }
}

fn main() {
    let mut calibrations: Vec<u16> = Vec::new();

    for line in std::fs::read_to_string("test1.txt").unwrap().lines() {
        let mut line_numbers: Vec<(usize, char)> = Vec::new();
        for (i, char) in line.chars().enumerate() {
            if char.is_ascii_digit() {
                line_numbers.push((i, char));
            }
        }

        fill_with_num(&mut line_numbers, line);
        line_numbers.sort_by(|a,b| a.0.cmp(&b.0));
dbg!(&line_numbers);

        let number = match line_numbers.len() {
            0 => "0".into(),
            1 => format!("{}{}", line_numbers[0].1, line_numbers[0].1),
            _ => format!(
                "{}{}",
                line_numbers.first().unwrap().1,
                line_numbers.last().unwrap().1
            ),
        };

        calibrations.push(number.parse().unwrap());
    }

    dbg!(&calibrations);
    println!("{}", calibrations.iter().sum::<u16>());
}
