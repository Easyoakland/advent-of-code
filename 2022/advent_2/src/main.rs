/*
The Elves begin to set up camp on the beach. To decide whose tent gets to be closest to the snack storage, a giant Rock Paper Scissors tournament is already in progress.

Rock Paper Scissors is a game between two players. Each game contains many rounds; in each round, the players each simultaneously choose one of Rock, Paper, or Scissors using a hand shape. Then, a winner for that round is selected: Rock defeats Scissors, Scissors defeats Paper, and Paper defeats Rock. If both players choose the same shape, the round instead ends in a draw.

Appreciative of your help yesterday, one Elf gives you an encrypted strategy guide (your puzzle input) that they say will be sure to help you win. "The first column is what your opponent is going to play: A for Rock, B for Paper, and C for Scissors. The second column--" Suddenly, the Elf is called away to help with someone's tent.

The second column, you reason, must be what you should play in response: X for Rock, Y for Paper, and Z for Scissors. Winning every time would be suspicious, so the responses must have been carefully chosen.

The winner of the whole tournament is the player with the highest score. Your total score is the sum of your scores for each round. The score for a single round is the score for the shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors) plus the score for the outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won).

Since you can't be sure if the Elf is trying to help you or trick you, you should calculate the score you would get if you were to follow the strategy guide.

For example, suppose you were given the following strategy guide:

A Y
B X
C Z

This strategy guide predicts and recommends the following:

    In the first round, your opponent will choose Rock (A), and you should choose Paper (Y). This ends in a win for you with a score of 8 (2 because you chose Paper + 6 because you won).
    In the second round, your opponent will choose Paper (B), and you should choose Rock (X). This ends in a loss for you with a score of 1 (1 + 0).
    The third round is a draw with both players choosing Scissors, giving you a score of 3 + 3 = 6.

In this example, if you were to follow the strategy guide, you would get a total score of 15 (8 + 1 + 6).

What would your total score be if everything goes exactly according to your strategy guide?

--- Part Two ---

The Elf finishes helping with the tent and sneaks back over to you. "Anyway, the second column says how the round needs to end: X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win. Good luck!"

The total score is still calculated in the same way, but now you need to figure out what shape to choose so the round ends as indicated. The example above now goes like this:

    In the first round, your opponent will choose Rock (A), and you need the round to end in a draw (Y), so you also choose Rock. This gives you a score of 1 + 3 = 4.
    In the second round, your opponent will choose Paper (B), and you choose Rock so you lose (X) with a score of 1 + 0 = 1.
    In the third round, you will defeat your opponent's Scissors with Rock for a score of 1 + 6 = 7.

Now that you're correctly decrypting the ultra top secret strategy guide, you would get a total score of 12.

Following the Elf's instructions for the second column, what would your total score be if everything goes exactly according to your strategy guide?

*/

use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    ops::Add,
};

fn main() -> Result<(), Box<dyn Error>> {
    part1::run()?;
    part2::run()?;
    Ok(())
}

#[derive(PartialEq, Clone, Copy)]
enum ChoiceKind {
    Rock,
    Paper,
    Scissors,
}

impl TryFrom<char> for ChoiceKind {
    type Error = String;
    fn try_from(value: char) -> Result<Self, String> {
        match value {
            'A' | 'X' => Ok(ChoiceKind::Rock),
            'B' | 'Y' => Ok(ChoiceKind::Paper),
            'C' | 'Z' => Ok(ChoiceKind::Scissors),
            c => Err(format!("Can't convert {c} to a ChoiceKind")),
        }
    }
}

impl ChoiceKind {
    fn to_value<T: From<u8>>(self) -> T {
        match self {
            ChoiceKind::Rock => 1.into(),
            ChoiceKind::Paper => 2.into(),
            ChoiceKind::Scissors => 3.into(),
        }
    }

    fn wins_against(self) -> ChoiceKind {
        match self {
            ChoiceKind::Rock => ChoiceKind::Scissors,
            ChoiceKind::Paper => ChoiceKind::Rock,
            ChoiceKind::Scissors => ChoiceKind::Paper,
        }
    }

    fn lose_against(self) -> ChoiceKind {
        match self {
            ChoiceKind::Scissors => ChoiceKind::Rock,
            ChoiceKind::Rock => ChoiceKind::Paper,
            ChoiceKind::Paper => ChoiceKind::Scissors,
        }
    }

    fn beats(self, other_choice: Self) -> bool {
        other_choice == self.wins_against()
    }
}
mod part1 {
    use super::*;
    pub fn run() -> Result<(), Box<dyn Error>> {
        let f = File::open("input.txt")?;
        let reader = BufReader::new(f);
        let mut result = 0;
        for line in reader.lines() {
            let line = line?;
            let other_choice = ChoiceKind::try_from(line.chars().nth(0).unwrap())?;
            let self_choice = ChoiceKind::try_from(line.chars().nth(2).unwrap())?;
            result += expected_value_of_round::<u32>(self_choice, other_choice);
        }
        println!("Part 1 answer: {result}");
        Ok(())
    }
}

mod part2 {
    use super::*;
    pub fn run() -> Result<(), Box<dyn Error>> {
        let f = File::open("input.txt")?;
        let reader = BufReader::new(f);
        let mut result = 0;

        for line in reader.lines() {
            let line = line?;
            let other_choice = ChoiceKind::try_from(line.chars().nth(0).unwrap())?;
            let self_choice = line.chars().nth(2).unwrap();
            let self_choice = match self_choice {
                'X' => other_choice.wins_against(),
                'Y' => other_choice,
                'Z' => other_choice.lose_against(),
                c => return Err(Box::from(format!("Can't convert {c} to a ChoiceKind"))),
            };
            result += expected_value_of_round::<u32>(self_choice, other_choice);
        }
        println!("Part 2 answer: {result}");
        Ok(())
    }
}

fn expected_value_of_round<T: From<u8> + Add<Output = T>>(
    self_choice: ChoiceKind,
    other_choice: ChoiceKind,
) -> T {
    let win_amount;
    if self_choice.beats(other_choice) {
        win_amount = 6;
    } else if other_choice.beats(self_choice) {
        win_amount = 0;
    } else {
        win_amount = 3;
    }
    self_choice.to_value::<T>() + win_amount.into()
}
