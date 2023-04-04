use std::collections::HashSet;
use std::fmt;

use clap::Parser;

/// The Result type for autorandr.
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The hidden code for this iteration of the game.
    #[arg(long, value_name = "HIDDEN_CODE")]
    hidden_code: String,
}

#[derive(Clone, Eq, PartialEq, Hash)]
enum Color {
    Red,
    Green,
    Blue,
    Purple,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = match self {
            Color::Red => 'r',
            Color::Green => 'g',
            Color::Blue => 'b',
            Color::Purple => 'p',
        };
        write!(f, "{}", c)
    }
}

#[derive(Clone)]
struct Code {
    positional: [Color; 4],
    set: HashSet<Color>,
}

impl Code {
    fn score(&self, other: Code) -> Score {
        let score: Vec<Key> = self
            .positional
            .iter()
            .zip(other.positional.iter())
            .map(|(s, o)| {
                if s == o {
                    Key::ColorAndPositionCorrect
                } else if self.set.contains(o) {
                    Key::ColorCorrect
                } else {
                    Key::Empty
                }
            })
            .collect();
        Score(
            score[0].clone(),
            score[1].clone(),
            score[2].clone(),
            score[3].clone(),
        )
    }
}

impl fmt::Display for Code {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            self.positional[0], self.positional[1], self.positional[2], self.positional[3]
        )
    }
}

impl TryFrom<String> for Code {
    type Error = Box<dyn std::error::Error>;

    fn try_from(s: String) -> Result<Self> {
        let mut set = HashSet::new();
        let pos: Vec<Color> = s
            .chars()
            .filter_map(|c| match c {
                '(' | ')' | ',' => None,
                'r' => {
                    set.insert(Color::Red);
                    Some(Color::Red)
                }
                'g' => {
                    set.insert(Color::Green);
                    Some(Color::Green)
                }
                'b' => {
                    set.insert(Color::Blue);
                    Some(Color::Blue)
                }
                'p' => {
                    set.insert(Color::Purple);
                    Some(Color::Purple)
                }
                _ => None,
            })
            .collect();
        match pos.len() {
            x if x < 4 => return Err(String::from("not enough characters").into()),
            x if x > 4 => return Err(String::from("too many characters").into()),
            _ => (),
        }
        Ok(Self {
            positional: [
                pos[0].clone(),
                pos[1].clone(),
                pos[2].clone(),
                pos[3].clone(),
            ],
            set,
        })
    }
}

struct Board {
    hidden_code: Code,
    rounds: Vec<Round>,
}

impl Board {
    fn get_input(&mut self) -> Result<bool> {
        println!("{}", &self);
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer)?;
        let code: Code = buffer.try_into()?;
        let round = Round {
            input_code: code.clone(),
            score: self.hidden_code.score(code),
        };

        self.rounds.push(round);

        match self.rounds.last() {
            Some(rs) => Ok(rs.wins()),
            None => Ok(false),
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for round in &self.rounds {
            write!(f, "{}", round)?;
        }
        Ok(())
    }
}

#[derive(Clone)]
enum Key {
    ColorCorrect,
    ColorAndPositionCorrect,
    Empty,
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = match self {
            Key::ColorCorrect => 'w',
            Key::ColorAndPositionCorrect => 'b',
            Key::Empty => ' ',
        };
        write!(f, "{}", c)
    }
}

struct Score(Key, Key, Key, Key);

impl Score {
    fn wins(&self) -> bool {
        match self {
            Score(
                Key::ColorAndPositionCorrect,
                Key::ColorAndPositionCorrect,
                Key::ColorAndPositionCorrect,
                Key::ColorAndPositionCorrect,
            ) => true,
            _ => false,
        }
    }
}

impl fmt::Display for Score {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {} {}", self.0, self.1, self.2, self.3)
    }
}

struct Round {
    input_code: Code,
    score: Score,
}

impl Round {
    fn wins(&self) -> bool {
        self.score.wins()
    }
}

impl fmt::Display for Round {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} - {}", self.input_code, self.score)
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut board = Board {
        hidden_code: cli.hidden_code.try_into()?,
        rounds: Vec::new(),
    };

    loop {
        if board.get_input()? {
            println!("{}", board);
            println!("congratulations, you win!");
            break;
        }
    }
    return Ok(());
}
