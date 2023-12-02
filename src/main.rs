use std::{env, fs};

fn main() {
    let filename = env::args().nth(1).expect("missing filename arg");
    let contents = fs::read_to_string(filename).expect("opening file failed");

    let number = contents
        .split('\n')
        // .inspect(|x| println!("x is {:?}", x))
        .fold(0, |x, line| {
            // println!("x is {:?}", x);
            let first_digit = line
                .chars()
                .find(|x| x.is_ascii_digit())
                .unwrap_or('0')
                .to_digit(10)
                .unwrap();
            let last_digit = line
                .chars()
                .rev()
                .find(|x| x.is_ascii_digit())
                .unwrap_or('0')
                .to_digit(10)
                .unwrap();
            x + first_digit * 10 + last_digit
            // let mut seen_first_digit: bool = false;

            // for char in line.chars() {
            //     if char.is_ascii_digit() {
            //         let incoming_digit = char.to_digit(10).unwrap();
            //         if !seen_first_digit {
            //             res += incoming_digit;
            //             seen_first_digit = true;
            //         } else if seen_first_digit {
            //             return res * 10 + incoming_digit + x
            //         }
            //     }
            // }
        });

    println!("{:?}", number);

    println!("begin part 2");

    let string_numbers = [
        "\n\n\n\n", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let part2 = contents.split('\n').fold(0, |calibration_sum, line| {
        let mut first_digit = 0;
        for (index, char) in line.char_indices() {
            if char.is_ascii_digit() {
                first_digit = char.to_digit(10).unwrap();
                break;
            } else if let Some(found_index) = string_numbers
                .iter()
                .position(|string_number| line.get(index..).unwrap().starts_with(string_number))
            {
                first_digit = found_index as u32;
                break;
            }
        }

        let mut last_digit = 0;
        for (index, char) in line.char_indices().rev() {
            if char.is_ascii_digit() {
                last_digit = char.to_digit(10).unwrap();
                break;
            } else if let Some(found_index) = string_numbers
                .iter()
                .position(|string_number| line.get(index..).unwrap().starts_with(string_number))
            {
                last_digit = found_index as u32;
                break;
            }
        }

        calibration_sum + first_digit * 10 + last_digit
    });

    println!("{:?}", part2);
}
