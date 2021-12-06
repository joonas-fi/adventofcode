use std::fs;
use std::str;

pub fn a() {
    println!(
        "power_consumption={}",
        power_consumption_from_report(
            fs::read_to_string("day03_input.txt").unwrap().lines(),
            "010001110001".len()
        )
    );
}

fn power_consumption(gamma_rate: i32, epsilon_rate: i32) -> i32 {
    return gamma_rate * epsilon_rate;
}

fn power_consumption_from_report(lines: str::Lines, bits_per_line: usize) -> i32 {
    let mut high_bit_counts = vec![0; bits_per_line];

    let mut sample_count = 0;

    for line in lines {
        let num = isize::from_str_radix(line, 2).unwrap();

        if line.len() != bits_per_line {
            panic!("unexpected bits_per_line");
        }

        for i in 0..bits_per_line {
            if (num & 1 << i) != 0 {
                high_bit_counts[i] = high_bit_counts[i] + 1
            }
        }

        sample_count = sample_count + 1;
    }

    let gamma_rate_bit = |i| {
        if high_bit_counts[i] >= sample_count / 2 {
            1 << i
        } else {
            0
        }
    };

    let epsilon_rate_bit = |i| {
        if high_bit_counts[i] < sample_count / 2 {
            1 << i
        } else {
            0
        }
    };

    let mut gamma_rate: i32 = 0;
    for n in 0..bits_per_line {
        gamma_rate = gamma_rate | gamma_rate_bit(n);
    }

    let mut epsilon_rate: i32 = 0;
    for n in 0..bits_per_line {
        epsilon_rate = epsilon_rate | epsilon_rate_bit(n);
    }

    return power_consumption(gamma_rate, epsilon_rate);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let test_input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
";

        assert_eq!(
            power_consumption_from_report(test_input.lines(), "00100".len()),
            198
        );
    }
}
