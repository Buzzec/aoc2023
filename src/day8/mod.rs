use bimap::BiMap;

const INPUT: &str = include_str!("input");
const TEST_INPUT: &str = include_str!("test_input");
const TEST_INPUT2: &str = include_str!("test_input2");
const TEST_INPUT3: &str = include_str!("test_input3");

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Network {
    steps: Vec<Direction>,
    node_name_map: BiMap<String, usize>,
    nodes: Vec<(usize, usize)>,
}
impl Network {
    pub fn parse(input: &str) -> Self {
        let mut lines = input.lines();
        let steps = lines
            .next()
            .unwrap()
            .chars()
            .map(|x| match x {
                'L' => Direction::Left,
                'R' => Direction::Right,
                x => panic!("Invalid direction: {}", x as u32),
            })
            .collect();
        lines.next().unwrap();

        let mut node_name_map = BiMap::new();
        let mut nodes = vec![];
        for line in lines {
            let mut parts = line.split_whitespace();
            let node = parts.next().unwrap();
            node_name_map.insert(node.to_string(), nodes.len());
            assert_eq!(parts.next().unwrap(), "=");
            let left_str = parts.next().unwrap();
            let left = left_str.trim().split_at(1).1.split_at(left_str.len() - 2).0;
            let right_str = parts.next().unwrap();
            let right = right_str.trim().split_at(right_str.len() - 1).0;
            nodes.push((left.to_string(), right.to_string()));
        }

        let nodes = nodes
            .into_iter()
            .map(|(left, right)| {
                (
                    *node_name_map.get_by_left(&left).unwrap(),
                    *node_name_map.get_by_left(&right).unwrap(),
                )
            })
            .collect();
        Self {
            steps,
            node_name_map,
            nodes,
        }
    }

    fn steps_needed(&self, start: &str, end: &str) -> usize {
        let start = *self.node_name_map.get_by_left(start).unwrap();
        let end = *self.node_name_map.get_by_left(end).unwrap();
        let mut steps = 0;
        let mut current = start;
        let mut step_iter = self.steps.iter().cycle();
        while current != end {
            let (left, right) = self.nodes[current];
            current = match step_iter.next().unwrap() {
                Direction::Left => left,
                Direction::Right => right,
            };
            steps += 1;
        }
        steps
    }

    fn nodes_ending_in_char(&self, char: char) -> impl Iterator<Item = usize> + '_ {
        self.node_name_map
            .iter()
            .filter(move |(name, _)| name.ends_with(char))
            .map(|(_, node)| *node)
    }

    fn steps_for_all(&self, starts: Vec<usize>, ends: Vec<usize>) -> usize {
        dbg!(&starts, &ends);

        let mut steps = 0;
        let mut step_iter = self.steps.iter().cycle();
        #[derive(Debug, Clone, Copy)]
        struct CycleLength {
            offset: usize,
            cycle_length: usize,
        }
        let mut cycle_lengths = vec![Option::<CycleLength>::None; starts.len()];
        let mut meta_data = vec![vec![Option::<usize>::None; self.nodes.len()]; starts.len()];
        let mut current = starts.iter().copied().map(Some).collect::<Vec<_>>();
        while current.iter().any(Option::is_some) {
            for (index, current) in current.iter_mut().enumerate() {
                if let Some(current_val) = *current {
                    let meta = &mut meta_data[index][current_val];
                    match meta {
                        Some(old_step) => {
                            dbg!((
                                index,
                                *old_step % self.steps.len(),
                                steps % self.steps.len()
                            ));
                            if *old_step % self.steps.len() == steps % self.steps.len() {
                                cycle_lengths[index] = Some(CycleLength {
                                    offset: *old_step,
                                    cycle_length: steps - *old_step,
                                });
                                *current = None;
                            }
                        }
                        None => *meta = Some(steps),
                    }
                }
            }

            let step = step_iter.next().unwrap();
            for current in &mut current.iter_mut().filter_map(|x| x.as_mut()) {
                let (left, right) = self.nodes[*current];
                match step {
                    Direction::Left => *current = left,
                    Direction::Right => *current = right,
                }
            }

            steps += 1;
        }
        let cycle_lengths = cycle_lengths
            .into_iter()
            .map(Option::unwrap)
            .collect::<Vec<_>>();
        println!("Metadata complete in {} steps: {meta_data:#?}", steps);
        println!("Cycle lengths: {:#?}", cycle_lengths);

        #[derive(Debug)]
        enum CycleType {
            InCycle {
                main_offset: usize,
                cycle_offset: usize,
                cycle_length: usize,
            },
            OutOfCycle {
                offset: usize,
            },
        }

        let cycle_ranges = meta_data
            .into_iter()
            .zip(cycle_lengths)
            .map(
                |(
                    meta_data,
                    CycleLength {
                        offset,
                        cycle_length,
                    },
                )| {
                    meta_data
                        .into_iter()
                        .enumerate()
                        .filter(|(index, _)| ends.contains(index))
                        .flat_map(|(index, meta)| meta)
                        .map(|meta| {
                            if meta < offset {
                                CycleType::OutOfCycle { offset: meta }
                            } else {
                                CycleType::InCycle {
                                    main_offset: meta,
                                    cycle_offset: meta - offset,
                                    cycle_length,
                                }
                            }
                        })
                        .collect::<Vec<_>>()
                },
            )
            .collect::<Vec<_>>();

        println!("{:#?}", cycle_ranges);
        todo!()
    }
}

#[cfg(test)]
#[test]
fn steps_for_all_test() {
    let network = Network::parse(INPUT);
    let steps1 = network.steps_needed("AAA", "ZZZ");
    let steps2 = network.steps_for_all(
        vec![*network.node_name_map.get_by_left("AAA").unwrap()],
        vec![*network.node_name_map.get_by_left("ZZZ").unwrap()],
    );
    assert_eq!(steps1, steps2);
}

pub fn day8() {
    let network = Network::parse(TEST_INPUT3);
    // let steps = network.steps_needed("AAA", "ZZZ");
    // println!("Day 8 part 1: {}", steps);

    let mut starts = network.nodes_ending_in_char('A').collect::<Vec<_>>();
    starts.sort();
    let mut ends = network.nodes_ending_in_char('Z').collect::<Vec<_>>();
    ends.sort();
    let steps = network.steps_for_all(starts, ends);
    println!("Day 8 part 2: {}", steps);
}
