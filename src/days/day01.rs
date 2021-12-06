#![allow(unused)]

use std::fs;

pub fn a() {
    let contents =
        fs::read_to_string("day01_input.txt").expect("Something went wrong reading the file");

    let mut lines = contents.split("\n");

    // start with -1 because the first item is incorrectly detected as increment
    let mut increases = -1;

    let mut previous = 0;
    for line in lines {
        if line == "" {
            continue;
        }

        let num = line.parse::<i32>().unwrap();

        if num > previous {
            increases = increases + 1;
        }

        previous = num;
    }

    println!("changes = {}", increases);
}

pub fn b() {
    let contents =
        fs::read_to_string("day01_input.txt").expect("Something went wrong reading the file");

    // could also use:
    //   let mut nums: Vec<i32> = contents.split("\n").map(|line| line.parse::<i32>().unwrap()).collect();
    // .. if I knew how to ignore empty lines in that spell
    let mut lines = contents.split("\n");

    let mut nums = Vec::new();
    for line in lines {
        if line == "" {
            continue;
        }

        nums.push(line.parse::<i32>().unwrap());
    }

    let window = |i| nums[i] + nums[i + 1] + nums[i + 2];

    let mut increases = 0;

    for n in 1..(nums.len() - 2) {
        let previous = window(n - 1);
        let current = window(n);

        if current > previous {
            increases = increases + 1;
        }
    }

    println!("changes = {}", increases);
}
