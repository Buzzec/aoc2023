use std::ops::{Add, AddAssign};
use std::str::FromStr;

const INPUT: &str = include_str!("input");
const TEST_INPUT: &str = include_str!("test_input");

#[derive(Debug, Default, Copy, Clone)]
struct CubeCount {
    red: u32,
    green: u32,
    blue: u32,
}
impl CubeCount {
    pub fn parse(input: &str) -> Self {
        let mut out = Self::default();
        for mut item in input.split(',') {
            item = item.trim();
            let (mut count, mut color) = item.split_at(item.find(' ').unwrap());
            count = count.trim();
            color = color.trim();
            let count = u32::from_str(count).unwrap();
            match color {
                "red" => out.red += count,
                "green" => out.green += count,
                "blue" => out.blue += count,
                _ => panic!("Unknown color: {}", color),
            }
        }
        out
    }
}
impl Add for CubeCount {
    type Output = CubeCount;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}
impl AddAssign for CubeCount {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
#[derive(Debug)]
struct Game {
    game_number: u32,
    pulls: Vec<CubeCount>,
}
impl Game {
    // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    pub fn parse(input: &str) -> Self {
        let (game, input) = input.split_at(5);
        assert_eq!("Game ", game);
        let (game_number, mut input) = input.split_at(input.find(':').unwrap());
        let game_number = u32::from_str(game_number).unwrap();
        input = input.split_at(1).1.trim();
        let pulls = input.split(";").map(CubeCount::parse).collect();
        Self { game_number, pulls }
    }

    pub fn is_possible(&self, max: CubeCount) -> bool {
        for pull in &self.pulls {
            if pull.red > max.red || pull.green > max.green || pull.blue > max.blue {
                return false;
            }
        }
        true
    }
}

pub fn day2() {
    const MAX_CUBES: CubeCount = CubeCount {
        red: 12,
        green: 13,
        blue: 14,
    };
    let games = INPUT.lines().map(Game::parse).collect::<Vec<_>>();

    let mut sum = 0;
    for game in games {
        if game.is_possible(MAX_CUBES) {
            sum += game.game_number;
        }
    }
    println!("Day 2 part 1: {}", sum);
}
