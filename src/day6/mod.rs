const INPUT: &str = include_str!("input");
const TEST_INPUT: &str = include_str!("test_input");

#[derive(Copy, Clone, Debug)]
pub struct Race {
    time: u64,
    distance_record: u64,
}
impl Race {
    pub fn parse(input: &str) -> Vec<Self> {
        let mut lines = input.lines();
        let mut times = lines
            .next()
            .unwrap()
            .split_at("Time:".len())
            .1
            .trim()
            .to_string();
        while times.contains("  ") {
            times = times.replace("  ", " ");
        }
        let times = times.split(' ').map(|x| x.trim().parse::<u64>().unwrap());
        let mut distances = lines
            .next()
            .unwrap()
            .split_at("Distance:".len())
            .1
            .trim()
            .to_string();
        while distances.contains("  ") {
            distances = distances.replace("  ", " ");
        }
        let distances = distances
            .split(' ')
            .map(|x| x.trim().parse::<u64>().unwrap());
        let mut out = vec![];
        for (time, distance) in times.zip(distances) {
            out.push(Self {
                time,
                distance_record: distance,
            });
        }
        out
    }

    pub fn parse_no_split(input: &str) -> Vec<Self> {
        let mut lines = input.lines();
        let mut times = lines
            .next()
            .unwrap()
            .split_at("Time:".len())
            .1
            .trim()
            .to_string();
        while times.contains(" ") {
            times = times.replace(" ", "");
        }
        let times = times.split(' ').map(|x| x.trim().parse::<u64>().unwrap());
        let mut distances = lines
            .next()
            .unwrap()
            .split_at("Distance:".len())
            .1
            .trim()
            .to_string();
        while distances.contains(" ") {
            distances = distances.replace(" ", "");
        }
        let distances = distances
            .split(' ')
            .map(|x| x.trim().parse::<u64>().unwrap());
        let mut out = vec![];
        for (time, distance) in times.zip(distances) {
            out.push(Self {
                time,
                distance_record: distance,
            });
        }
        out
    }

    pub fn solve_race_for_distance(&self, distance: u64) -> [f64; 2] {
        let sqrt = ((self.time as f64).powi(2) - 4.0 * distance as f64).sqrt();
        [
            0.5 * (self.time as f64 - sqrt),
            0.5 * (self.time as f64 + sqrt),
        ]
    }

    pub fn solve_race(&self) -> [f64; 2] {
        self.solve_race_for_distance(self.distance_record)
    }

    pub fn winning_count(&self) -> u64 {
        let [min, max] = self.solve_race();
        ((max - 0.01).floor() as u64).min(self.time) - ((min + 0.01).floor() as u64).max(0)
    }
}

pub fn day6() {
    let races = Race::parse(INPUT);
    let win_counts = races.iter().map(Race::winning_count).collect::<Vec<_>>();
    println!("Day 6 part 1: {}", win_counts.iter().product::<u64>());

    let races = Race::parse_no_split(INPUT);
    let win_counts = races.iter().map(Race::winning_count).collect::<Vec<_>>();
    println!("Day 6 part 2: {}", win_counts.iter().product::<u64>());
}
