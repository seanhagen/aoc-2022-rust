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

const POINTS_ROCK: i32 = 1;
const POINTS_PAPER: i32 = 2;
const POINTS_SCISSOR: i32 = 3;

const POINTS_LOSE: i32 = 0;
const POINTS_DRAW: i32 = 3;
const POINTS_WIN: i32 = 6;

#[derive(PartialEq, Debug)]
enum Hand {
    Rock,
    Paper,
    Scissor,
}

impl Hand {
    fn parse(input: &str) -> Hand {
        match input {
            "A" => Hand::Rock,
            "X" => Hand::Rock,
            "B" => Hand::Paper,
            "Y" => Hand::Paper,
            "C" => Hand::Scissor,
            _ => Hand::Scissor,
        }
    }

    fn points(&self) -> i32 {
        match self {
            Hand::Rock => POINTS_ROCK,
            Hand::Paper => POINTS_PAPER,
            Hand::Scissor => POINTS_SCISSOR,
        }
    }

    fn points_vs(&self, other: Hand) -> Game {
        match self {
            Hand::Rock => match other {
                Hand::Rock => Game::Draw(POINTS_DRAW + self.points()),
                Hand::Scissor => Game::Win(POINTS_WIN + self.points()),
                _ => Game::Loss(POINTS_LOSE + self.points()),
            },
            Hand::Paper => match other {
                Hand::Paper => Game::Draw(POINTS_DRAW + self.points()),
                Hand::Rock => Game::Win(POINTS_WIN + self.points()),
                _ => Game::Loss(POINTS_LOSE + self.points()),
            },
            _ => match other {
                Hand::Scissor => Game::Draw(POINTS_DRAW + self.points()),
                Hand::Paper => Game::Win(POINTS_WIN + self.points()),
                _ => Game::Loss(POINTS_LOSE + self.points()),
            },
        }
    }
}

#[derive(PartialEq, Debug)]
enum Game {
    Draw(i32),
    Win(i32),
    Loss(i32),
}

struct Elf {
    play: Hand,
}

struct Me {
    play: Hand,
}

trait Player {
    type Opponent;
    fn result_against(&self, other: Self::Opponent) -> Game;
}

impl Player for Me {
    type Opponent = Elf;
    fn result_against(&self, other: Self::Opponent) -> Game {
        self.play.points_vs(other.play)
    }
}

pub struct RockPaperScissor {
    score: i32,
}

impl RockPaperScissor {
    pub fn new() -> RockPaperScissor {
        RockPaperScissor { score: 0 }
    }

    pub fn score(&self) -> i32 {
        self.score
    }

    pub fn add(&mut self, line: &str) {
        let parts = line.split(" ").collect::<Vec<&str>>();

        let elf = Elf {
            play: Hand::parse(parts[0]),
        };
        let player = Me {
            play: Hand::parse(parts[1]),
        };

        self.score += match player.result_against(elf) {
            Game::Draw(points) => points,
            Game::Win(points) => points,
            Game::Loss(points) => points,
        }

        // let outcome_points = match parts[0] {
        //     "A" => {
        //         // elf plays Rock
        //         match parts[1] {
        //             "X" => 3, // i play rock
        //             "Y" => 6, // i play paper
        //             "Z" => 0, // i play scissors
        //             _ => -1,  // undefined
        //         }
        //     }
        //     "B" => {
        //         // elf plays Paper
        //         match parts[1] {
        //             "X" => 0, // i play rock
        //             "Y" => 3, // i play paper
        //             "Z" => 6, // i play scissors
        //             _ => -1,  // undefined
        //         }
        //     }
        //     "C" => {
        //         // elf plays Scissors
        //         match parts[1] {
        //             "X" => 6, // i play rock
        //             "Y" => 0, // i play paper
        //             "Z" => 3, // i play scissors
        //             _ => -1,  // undefined
        //         }
        //     }
        //     _ => -1, // undefined
        // };
        // let play_points = match parts[1] {
        //     "X" => 1, // i play rock
        //     "Y" => 2, // i play paper
        //     "Z" => 3, // i play scissors
        //     _ => -1,  //undefined
        // };
        // println!("parts.0 => {}", parts[0]);
        // println!("parts.1 => {}", parts[1]);
        // println!("outcome points: {outcome_points}");
        // println!("play points: {play_points}");
        // self.score += outcome_points + play_points;
    }

    pub fn score(&self) -> i32 {
        self.score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_hand_points() {
        let h = Hand::Rock;
        assert_eq!(1, h.points());

        let h = Hand::Paper;
        assert_eq!(2, h.points());

        let h = Hand::Scissor;
        assert_eq!(3, h.points());
    }

    #[test]
    fn correct_game_results() {
        let m = Me { play: Hand::Rock };
        let e = Elf { play: Hand::Rock };
        assert_eq!(Game::Draw(4), m.result_against(e));

        let e = Elf { play: Hand::Paper };
        assert_eq!(Game::Loss(1), m.result_against(e));

        let e = Elf {
            play: Hand::Scissor,
        };
        assert_eq!(Game::Win(7), m.result_against(e));

        let m = Me { play: Hand::Paper };
        let e = Elf { play: Hand::Paper };
        assert_eq!(Game::Draw(5), m.result_against(e));

        let e = Elf { play: Hand::Rock };
        assert_eq!(Game::Win(8), m.result_against(e));

        let e = Elf {
            play: Hand::Scissor,
        };
        assert_eq!(Game::Loss(2), m.result_against(e));

        let m = Me {
            play: Hand::Scissor,
        };
        let e = Elf {
            play: Hand::Scissor,
        };
        assert_eq!(Game::Draw(6), m.result_against(e));

        let e = Elf { play: Hand::Rock };
        assert_eq!(Game::Loss(3), m.result_against(e));

        let e = Elf { play: Hand::Paper };
        assert_eq!(Game::Win(9), m.result_against(e));
    }

    #[test]
    fn no_lines_added_has_zero_score() {
        let game = RockPaperScissor::new();
        assert_eq!(0, game.score());
    }

    #[test]
    fn parse_hand_str() {
        assert_eq!(Hand::Rock, Hand::parse("A"));
        assert_eq!(Hand::Rock, Hand::parse("X"));
        assert_eq!(Hand::Paper, Hand::parse("B"));
        assert_eq!(Hand::Paper, Hand::parse("Y"));
        assert_eq!(Hand::Scissor, Hand::parse("C"));
        assert_eq!(Hand::Scissor, Hand::parse("Z"));
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
