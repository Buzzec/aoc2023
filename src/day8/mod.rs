use bimap::BiMap;

const INPUT: &str = include_str!("input");
const TEST_INPUT: &str = include_str!("test_input");
const TEST_INPUT2: &str = include_str!("test_input2");

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
}

pub fn day8() {
    let network = Network::parse(INPUT);
    println!("Day 8 part 1: {}", network.steps_needed("AAA", "ZZZ"));
}
