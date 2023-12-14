use grid::Grid;

const INPUT: &str = include_str!("input");
const TEST_INPUT: &str = include_str!("test_input");

#[derive(Debug)]
pub struct EngineSchematic {
    grid: Grid<GridCell>,
}
impl EngineSchematic {
    pub fn parse(input: &str) -> Self {
        let mut grid = Grid::new(input.lines().count(), input.lines().next().unwrap().len());
        for (y, line) in input.lines().enumerate() {
            for (x, char) in line.chars().enumerate() {
                let cell = match char {
                    x if x.is_numeric() => GridCell::Number(x.to_digit(10).unwrap()),
                    '.' => GridCell::Empty,
                    x => GridCell::Symbol(x),
                };
                *grid.get_mut(x, y).unwrap() = cell;
            }
        }
        Self { grid }
    }

    pub fn possible_part_numbers(&self) -> Vec<PartNumber> {
        let mut out = vec![];
        for (y, row) in self.grid.iter_cols().enumerate() {
            let mut current_number: Option<PartNumber> = None;
            for (x, cell) in row.enumerate() {
                if let GridCell::Number(value) = cell {
                    if let Some(part_number) = &mut current_number {
                        part_number.length += 1;
                        part_number.value *= 10;
                        part_number.value += *value;
                    } else {
                        current_number = Some(PartNumber {
                            x,
                            y,
                            length: 1,
                            value: *value,
                        });
                    }
                } else {
                    if let Some(val) = current_number.take() {
                        out.push(val);
                    }
                }
            }
            if let Some(val) = current_number {
                out.push(val);
            }
        }
        out
    }

    pub fn part_number_near_symbol(&self, part_number: PartNumber) -> bool {
        for x in part_number.x.saturating_sub(1)
            ..(part_number.x + part_number.length + 1).min(self.grid.rows())
        {
            for y in part_number.y.saturating_sub(1)..(part_number.y + 2).min(self.grid.cols()) {
                if let GridCell::Symbol(_) = self.grid.get(x, y).unwrap() {
                    return true;
                }
            }
        }
        false
    }

    pub fn part_numbers(&self) -> impl Iterator<Item = PartNumber> + '_ {
        self.possible_part_numbers()
            .into_iter()
            .filter(|x| self.part_number_near_symbol(*x))
    }

    pub fn gears(&self) -> Vec<Gear> {
        let part_numbers = self.part_numbers().collect::<Vec<_>>();
        self.grid
            .iter_cols()
            .enumerate()
            .flat_map(|(y, row)| row.enumerate().map(move |(x, cell)| (x, y, cell)))
            .filter(|(_, _, cell)| matches!(cell, GridCell::Symbol('*')))
            .filter_map(|(x, y, _)| {
                let mut parts = vec![];
                for part_number in &part_numbers {
                    // check if part is adjacent/diagonal to symbol
                    if ((part_number.x as isize - x as isize).abs() <= 1
                        || ((part_number.x + part_number.length - 1) as isize - x as isize).abs()
                            <= 1
                        || (x >= part_number.x && x < part_number.x + part_number.length))
                        && (part_number.y as isize - y as isize).abs() <= 1
                    {
                        parts.push(*part_number);
                    }
                }
                parts
                    .as_slice()
                    .try_into()
                    .ok()
                    .map(|parts| Gear { parts, x, y })
            })
            .collect()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct PartNumber {
    x: usize,
    y: usize,
    length: usize,
    value: u32,
}
#[derive(Debug, Copy, Clone)]
pub struct Gear {
    parts: [PartNumber; 2],
    x: usize,
    y: usize,
}
impl Gear {
    pub fn ratio(&self) -> u32 {
        self.parts[0].value * self.parts[1].value
    }
}

#[derive(Debug, Default)]
pub enum GridCell {
    #[default]
    Empty,
    Symbol(char),
    Number(u32),
}
impl GridCell {
    pub fn is_number(&self) -> bool {
        matches!(self, Self::Number(_))
    }
}

pub fn day3() {
    let engine = EngineSchematic::parse(INPUT);
    let sum = engine.part_numbers().map(|x| x.value).sum::<u32>();
    println!("Day 3 part 1: {}", sum);

    let gears = engine.gears();
    let sum = gears.iter().map(Gear::ratio).sum::<u32>();
    println!("Day 3 part 2: {}", sum);
}
