/*
Elf:
A -> Rock
B -> Paper
C -> Scissor

Me:
X -> Rock
Y -> Paper
Z -> Scissor

Points:
Rock    -> 1
Paper   -> 2
Scissor -> 3

Lose -> 0
Draw -> 3
Win  -> 6
*/

pub struct RockPaperScissor {
    score: i32,
}

#[derive(PartialEq, Debug)]
enum Hand {
    Rock,
    Paper,
    Scissor,
}

struct Elf {
    play: Hand,
}

struct Me {
    play: Hand,
}

trait Player {
    fn result_against<T: Player>(&self, other: T) -> Game;
}

impl Player for Elf {
    fn result_against<Me>(&self, other: Me) -> Game {
        Game::Draw
    }
}

impl Player for Me {
    fn result_against<Elf>(&self, other: Elf) -> Game {
        Game::Draw
    }
}

#[derive(PartialEq, Debug)]
enum Game {
    Draw,
    Win,
    Loss,
}

impl RockPaperScissor {
    pub fn new() -> RockPaperScissor {
        RockPaperScissor { score: 0 }
    }

    pub fn add(&mut self, line: &str) {
        let parts = line.split(" ").collect::<Vec<&str>>();
        let outcome_points = match parts[0] {
            "A" => {
                // elf plays Rock
                match parts[1] {
                    "X" => 3, // i play rock
                    "Y" => 6, // i play paper
                    "Z" => 0, // i play scissors
                    _ => -1,  // undefined
                }
            }
            "B" => {
                // elf plays Paper
                match parts[1] {
                    "X" => 0, // i play rock
                    "Y" => 3, // i play paper
                    "Z" => 6, // i play scissors
                    _ => -1,  // undefined
                }
            }
            "C" => {
                // elf plays Scissors
                match parts[1] {
                    "X" => 6, // i play rock
                    "Y" => 0, // i play paper
                    "Z" => 3, // i play scissors
                    _ => -1,  // undefined
                }
            }
            _ => -1, // undefined
        };
        let play_points = match parts[1] {
            "X" => 1, // i play rock
            "Y" => 2, // i play paper
            "Z" => 3, // i play scissors
            _ => -1,  //undefined
        };
        println!("parts.0 => {}", parts[0]);
        println!("parts.1 => {}", parts[1]);
        println!("outcome points: {outcome_points}");
        println!("play points: {play_points}");
        self.score += outcome_points + play_points;
    }

    pub fn score(&self) -> i32 {
        self.score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_something() {
        let p = Me { play: Hand::Rock };
        let u = Me { play: Hand::Rock };
        let result = p.result_against(u);
        assert_eq!(Game::Draw, result)
    }

    #[test]
    fn no_lines_added_has_zero_score() {
        let game = RockPaperScissor::new();
        assert_eq!(0, game.score());
    }

    #[test]
    fn example_calculated_properly() {
        let input = vec!["A Y", "B X", "C Z"];
        let mut game = RockPaperScissor::new();

        for line in input {
            game.add(line);
        }

        assert_eq!(15, game.score());
    }

    #[test]
    fn another_input_calculates_proerly() {
        let mut game = RockPaperScissor::new();

        game.add("B Z");
        assert_eq!(9, game.score());

        game.add("A Y");
        assert_eq!(17, game.score());

        game.add("B X");
        assert_eq!(18, game.score());
    }
}
