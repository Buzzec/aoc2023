const INPUT: &str = include_str!("input");
const TEST_INPUT: &str = include_str!("test_input");

#[derive(Copy, Clone, Debug)]
struct AlmanacRange {
    destination_start: u64,
    source_start: u64,
    length: u64,
}
impl AlmanacRange {
    pub fn parse(input: &str) -> Self {
        let mut vals = input.split(' ');
        Self {
            destination_start: vals.next().unwrap().parse().unwrap(),
            source_start: vals.next().unwrap().parse().unwrap(),
            length: vals.next().unwrap().parse().unwrap(),
        }
    }

    pub fn map_value(&self, value: u64) -> Option<u64> {
        if value < self.source_start || value >= self.source_start + self.length {
            None
        } else {
            Some(self.destination_start + value - self.source_start)
        }
    }
}

#[derive(Debug, Default)]
struct AlmanacRanges {
    ranges: Vec<AlmanacRange>,
}
impl AlmanacRanges {
    pub fn parse<'a>(input: &mut impl Iterator<Item = &'a str>) -> Self {
        let mut out = Self::default();
        loop {
            let line = match input.next() {
                Some(line) => line,
                None => break,
            };
            if line.is_empty() {
                break;
            }
            out.ranges.push(AlmanacRange::parse(line));
        }
        out
    }

    pub fn map_value(&self, value: u64) -> u64 {
        for range in &self.ranges {
            if let Some(mapped_value) = range.map_value(value) {
                return mapped_value;
            }
        }
        value
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
    seed_to_soil: AlmanacRanges,
    soil_to_fertilizer: AlmanacRanges,
    fertilizer_to_water: AlmanacRanges,
    water_to_light: AlmanacRanges,
    light_to_temperature: AlmanacRanges,
    temperature_to_humidity: AlmanacRanges,
    humidity_to_location: AlmanacRanges,
}
impl Almanac {
    pub fn parse(input: &str) -> Self {
        let mut lines = input.lines();
        let seeds = lines
            .next()
            .unwrap()
            .split_at("seeds: ".len())
            .1
            .trim()
            .split(' ')
            .map(|x| x.parse().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(lines.next(), Some(""));
        assert_eq!(lines.next(), Some("seed-to-soil map:"));
        let seed_to_soil = AlmanacRanges::parse(&mut lines);
        assert_eq!(lines.next(), Some("soil-to-fertilizer map:"));
        let soil_to_fertilizer = AlmanacRanges::parse(&mut lines);
        assert_eq!(lines.next(), Some("fertilizer-to-water map:"));
        let fertilizer_to_water = AlmanacRanges::parse(&mut lines);
        assert_eq!(lines.next(), Some("water-to-light map:"));
        let water_to_light = AlmanacRanges::parse(&mut lines);
        assert_eq!(lines.next(), Some("light-to-temperature map:"));
        let light_to_temperature = AlmanacRanges::parse(&mut lines);
        assert_eq!(lines.next(), Some("temperature-to-humidity map:"));
        let temperature_to_humidity = AlmanacRanges::parse(&mut lines);
        assert_eq!(lines.next(), Some("humidity-to-location map:"));
        let humidity_to_location = AlmanacRanges::parse(&mut lines);
        assert_eq!(lines.next(), None);

        Self {
            seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        }
    }
}

pub fn day5() {
    let almanac = Almanac::parse(INPUT);
    let location_numbers = almanac
        .seeds
        .iter()
        .copied()
        .map(|x| almanac.seed_to_soil.map_value(x))
        .map(|x| almanac.soil_to_fertilizer.map_value(x))
        .map(|x| almanac.fertilizer_to_water.map_value(x))
        .map(|x| almanac.water_to_light.map_value(x))
        .map(|x| almanac.light_to_temperature.map_value(x))
        .map(|x| almanac.temperature_to_humidity.map_value(x))
        .map(|x| almanac.humidity_to_location.map_value(x))
        .collect::<Vec<_>>();
    println!("Day 5 part 1: {:?}", location_numbers.iter().min().unwrap());
}
