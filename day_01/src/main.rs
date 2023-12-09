use std::fs;

struct CharItem {
    value: u32,
    ignore: bool,
}

fn extract_number(line: String) -> u32 {
    const RADIX: u32 = 10;

    let mut char_list: Vec<CharItem> = Vec::new();

    for item in line.chars() {
        let resp = item.to_digit(RADIX);

        let number = match resp {
            Some(number) => CharItem { value: number, ignore: false },
            None => CharItem { value: 0, ignore: true }
        };
        char_list.push(number);
    }

    let first_digit: u32;
    let second_digit: u32;

    let mut filtered_numbers: Vec<u32> = Vec::new();

    for item in char_list {
        if item.ignore == false {
            filtered_numbers.push(item.value)
        }
    }

    first_digit = filtered_numbers[0];
    second_digit = filtered_numbers[filtered_numbers.len() - 1];

    let result_str = format!("{}{}", first_digit, second_digit);

    let result = result_str.parse().unwrap();

    println!("{} : {}", line, result);

    result
}

fn main() {
    println!("<Day 01>");
    let file_contents = fs::read_to_string("data/001_data.txt").expect("ok");

    let mut calibration_values: Vec<u32> = Vec::new();

    for line in file_contents.split("\n") {
        calibration_values.push(extract_number(String::from(line)));
    }

    let sum_of_calibration_values: u32 = calibration_values.iter().sum();

    print!("sum_of_calibration_values : {}", sum_of_calibration_values);
}
