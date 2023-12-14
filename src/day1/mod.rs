use std::cmp::Reverse;
use std::iter::Rev;

const INPUT: &str = include_str!("input");
const TEST_INPUT: &str = include_str!("test_input");

const LINE_MAP: &[(&str, char)] = &[
    ("one", '1'),
    ("two", '2'),
    ("three", '3'),
    ("four", '4'),
    ("five", '5'),
    ("six", '6'),
    ("seven", '7'),
    ("eight", '8'),
    ("nine", '9'),
];

pub fn day1() {
    let mut sum = 0;
    for line in INPUT.lines() {
        // for line in TEST_INPUT.lines() {
        let mut line = line.to_string();
        let mut to_add = vec![];
        for (word, num) in LINE_MAP {
            if let Some(index) = line.find(word) {
                to_add.push((index, *num));
            }
            if let Some(index) = line.rfind(word) {
                to_add.push((index, *num));
            }
        }
        to_add.sort_by_key(|(index, _)| Reverse(*index));
        for (index, num) in to_add {
            line.insert(index, num);
        }

        let mut first = None;
        let mut last = None;
        for char in line.chars() {
            if char.is_numeric() {
                if first.is_none() {
                    first = Some(char);
                }
                last = Some(char);
            }
        }
        if let (Some(first), Some(last)) = (first, last) {
            let add = first.to_digit(10).unwrap() * 10 + last.to_digit(10).unwrap();
            println!("{} = {}", line, add);
            sum += add;
        } else {
            panic!("Line has no numbers");
        }
    }
    println!("Day 1 part 1: {}", sum);
}
