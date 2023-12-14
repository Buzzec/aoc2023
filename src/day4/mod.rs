const INPUT: &str = include_str!("input");
const TEST_INPUT: &str = include_str!("test_input");

#[derive(Debug)]
pub struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    my_numbers: Vec<u32>,
}
impl Card {
    pub fn parse(input: &str) -> Self {
        let (card, numbers) = input.split_at(input.find(':').unwrap());
        let numbers = numbers.split_at(1).1.trim();
        let (card, id) = card.split_at(5);
        assert_eq!("Card ", card);
        let (winning, my) = numbers.split_at(numbers.find('|').unwrap());
        let winning = winning.trim();
        let my = my.split_at(1).1.trim();
        Self {
            id: id.trim().parse().unwrap(),
            winning_numbers: winning
                .trim()
                .replace("  ", " ")
                .split(' ')
                .map(|x| x.trim().parse().unwrap())
                .collect(),
            my_numbers: my
                .trim()
                .replace("  ", " ")
                .split(' ')
                .map(|x| x.trim().parse().unwrap())
                .collect(),
        }
    }

    pub fn winning_count(&self) -> u32 {
        let mut winning_count: u32 = 0;
        for my_number in &self.my_numbers {
            if self.winning_numbers.contains(my_number) {
                winning_count += 1;
            }
        }
        winning_count
    }

    pub fn value(&self) -> u32 {
        self.winning_count()
            .checked_sub(1)
            .map(|x| 1 << x)
            .unwrap_or(0)
    }
}

pub fn day4() {
    let cards = INPUT.lines().map(Card::parse).collect::<Vec<_>>();
    let sum = cards.iter().map(Card::value).sum::<u32>();
    println!("Day 4 part 1: {}", sum);

    let mut card_multipliers = (0..cards.len()).map(|_| 1u32).collect::<Vec<_>>();
    for (index, card) in cards.iter().enumerate() {
        let multiplier = card_multipliers[index];
        for other_index in (index + 1).min(card_multipliers.len())
            ..=(index + card.winning_count() as usize).min(card_multipliers.len() - 1)
        {
            card_multipliers[other_index] += multiplier;
        }
    }
    let sum = card_multipliers.iter().sum::<u32>();
    println!("Day 4 part 2: {}", sum);
}
