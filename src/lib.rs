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

    fn parse_outcome(elf: &Hand, me: &str) -> Hand {
        match elf {
            Hand::Rock => match me {
                "X" => Hand::Scissor,
                "Y" => Hand::Rock,
                _ => Hand::Paper,
            },
            Hand::Paper => match me {
                "X" => Hand::Rock,
                "Y" => Hand::Paper,
                _ => Hand::Scissor,
            },
            Hand::Scissor => match me {
                "X" => Hand::Paper,
                "Y" => Hand::Scissor,
                _ => Hand::Rock,
            },
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

impl Game {
    fn parse(elf: &str, me: &str) -> Game {
        let elf = Hand::parse(elf);
        let me = Hand::parse_outcome(&elf, me);

        match elf {
            Hand::Rock => match me {
                Hand::Rock => Game::Draw(POINTS_DRAW + me.points()),
                Hand::Paper => Game::Win(POINTS_WIN + me.points()),
                Hand::Scissor => Game::Loss(POINTS_LOSE + me.points()),
            },
            Hand::Paper => match me {
                Hand::Paper => Game::Draw(POINTS_DRAW + me.points()),
                Hand::Scissor => Game::Win(POINTS_WIN + me.points()),
                Hand::Rock => Game::Loss(POINTS_LOSE + me.points()),
            },
            Hand::Scissor => match me {
                Hand::Scissor => Game::Draw(POINTS_DRAW + me.points()),
                Hand::Rock => Game::Win(POINTS_WIN + me.points()),
                Hand::Paper => Game::Loss(POINTS_LOSE + me.points()),
            },
        }
    }
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

    pub fn add_correct(&mut self, line: &str) {
        let parts = line.split(" ").collect::<Vec<&str>>();

        let elf = Elf {
            play: Hand::parse(parts[0]),
        };

        let player = Me {
            play: Hand::parse_outcome(&elf.play, parts[1]),
        };

        self.score += match player.result_against(elf) {
            Game::Draw(points) => points,
            Game::Win(points) => points,
            Game::Loss(points) => points,
        }
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

    #[test]
    fn day_two_calculated_properly() {
        let input = vec!["A Y", "B X", "C Z"];
        let mut game = RockPaperScissor::new();

        for line in input {
            game.add_correct(line);
        }

        assert_eq!(12, game.score());
    }

    #[test]
    fn day_two_parse_input() {
        assert_eq!(Game::Draw(4), Game::parse("A", "Y"));
        assert_eq!(Game::Loss(1), Game::parse("B", "X"));
        assert_eq!(Game::Win(7), Game::parse("C", "Z"));
    }

    #[test]
    fn day_two_parse_outcome() {
        // X -> i lose
        // Y -> draw
        // Z -> i win

        let elf = Hand::Rock;
        assert_eq!(Hand::Scissor, Hand::parse_outcome(&elf, "X"));
        assert_eq!(Hand::Rock, Hand::parse_outcome(&elf, "Y"));
        assert_eq!(Hand::Paper, Hand::parse_outcome(&elf, "Z"));

        let elf = Hand::Paper;
        assert_eq!(Hand::Rock, Hand::parse_outcome(&elf, "X"));
        assert_eq!(Hand::Paper, Hand::parse_outcome(&elf, "Y"));
        assert_eq!(Hand::Scissor, Hand::parse_outcome(&elf, "Z"));

        let elf = Hand::Scissor;
        assert_eq!(Hand::Paper, Hand::parse_outcome(&elf, "X"));
        assert_eq!(Hand::Scissor, Hand::parse_outcome(&elf, "Y"));
        assert_eq!(Hand::Rock, Hand::parse_outcome(&elf, "Z"));
    }
}
