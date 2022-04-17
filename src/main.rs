#[macro_use]
extern crate crossterm;

use clap::Parser;
use crossterm::cursor;
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use std::io::{stdout, Stdout, Write};

//Mastermind

#[derive(Debug, PartialEq)]
enum CodeCell {
    R, //Red
    G, //Green
    B, //Blue
    Y, //Yellow
    M, //Magenta
    C, //Cyan
    E, //Empty
}
impl std::fmt::Display for CodeCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            CodeCell::R => write!(f, "\x1b[31;41m \x1b[0m"),
            CodeCell::G => write!(f, "\x1b[32;42m \x1b[0m"),
            CodeCell::B => write!(f, "\x1b[34;44m \x1b[0m"),
            CodeCell::Y => write!(f, "\x1b[33;43m \x1b[0m"),
            CodeCell::M => write!(f, "\x1b[35;45m \x1b[0m"),
            CodeCell::C => write!(f, "\x1b[36;46m \x1b[0m"),
            CodeCell::E => write!(f, "\x1b[0m "),
        }
    }
}
impl Distribution<CodeCell> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> CodeCell {
        match rng.gen_range(0..6) {
            0 => CodeCell::R,
            1 => CodeCell::G,
            2 => CodeCell::B,
            3 => CodeCell::Y,
            4 => CodeCell::M,
            _ => CodeCell::C,
        }
    }
}

#[derive(Debug)]
struct Code(Vec<CodeCell>);
impl std::fmt::Display for Code {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s: String = "".to_owned();
        for c in &self.0 {
            s += &c.to_string();
        }
        f.write_str(&s)
    }
}
impl Code {
    pub fn new(peg_count: u8) -> Code {
        let mut vector: Vec<CodeCell> = Vec::new();
        for _ in 0..peg_count {
            vector.push(rand::random());
        }
        Code { 0: vector }
    }
}

#[derive(Debug, PartialEq)]
enum ScoreCell {
    W, //White
    K, //Black
}
impl std::fmt::Display for ScoreCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ScoreCell::W => write!(f, "\x1b[37;47m \x1b[0m"),
            ScoreCell::K => write!(f, "\x1b[91;101m \x1b[0m"),
        }
    }
}
#[derive(Debug, PartialEq)]
struct Score(Vec<Option<ScoreCell>>);
impl std::fmt::Display for Score {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s: String = "".to_owned();
        for c in &self.0 {
            match c {
                Some(c) => s += &c.to_string(),
                None => s += " ",
            }
        }
        f.write_str(&s)
    }
}
impl Score {
    pub fn new(peg_count: u8) -> Score {
        let mut vector: Vec<Option<ScoreCell>> = Vec::new();
        for _ in 0..peg_count {
            vector.push(None);
        }
        Score { 0: vector }
    }
}

#[derive(Debug)]
struct MasterMind {
    code: Code,
    peg_count: usize,
    guesses: u8,
}
impl MasterMind {
    pub fn new(peg_count: u8) -> MasterMind {
        MasterMind {
            code: Code::new(peg_count),
            peg_count: peg_count as usize,
            guesses: 0,
        }
    }
    pub fn set_code(&mut self, code: Code) {
        self.code = code;
    }
    pub fn guess(&self, guess: &Code) -> Score {
        let mut score = Score::new(self.peg_count.try_into().unwrap());

        for _ in 0..self.peg_count {
            score.0.push(None);
        }

        let mut used_code = Vec::new();
        let mut used_guess = Vec::new();
        let mut in_position = 0;
        let mut in_code = 0;
        //Scan for correct position
        for _ in 0..self.peg_count {
            used_code.push(false);
            used_guess.push(false);
        }

        //Scan for correct position
        for n in 0..self.peg_count {
            if guess.0[n] == self.code.0[n] {
                used_code[n] = true;
                used_guess[n] = true;
                in_position += 1;
            }
        }
        //Scan for used in code, but not already used for position
        for n in 0..self.peg_count {
            if !used_guess[n] {
                for n_code in 0..self.peg_count {
                    if !used_code[n] && guess.0[n] == self.code.0[n_code] {
                        used_code[n_code] = true;
                        used_guess[n] = true;
                        in_code += 1;
                    }
                }
            }
        }

        //Build result
        for n in 0..self.peg_count {
            if in_position > 0 {
                score.0[n] = Some(ScoreCell::K);
                in_position -= 1;
            } else if in_code > 0 {
                score.0[n] = Some(ScoreCell::W);
                in_code -= 1;
            }
        }

        score
    }
    pub fn get_guess(&self, stdout: &mut Stdout) -> Result<Code, &'static str> {
        ////clearing the screen, going to top left corner and printing welcoming message
        //execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0), Print(r#"ctrl + q to exit, ctrl + h to print "Hello world", alt + t to print "crossterm is cool""#))
        //    .unwrap();

        //execute!(stdout, Print(r#"ctrl + q to exit, ctrl + h to print "Hello world", alt + t to print "crossterm is cool""#))
        //    .unwrap();

        let mut guesses: Vec<CodeCell> = vec![];
        while guesses.len() < self.peg_count {
            //going to top left corner
            //execute!(stdout, cursor::MoveTo(0, 0)).unwrap();

            match read().unwrap() {
                Event::Key(KeyEvent {
                    code: KeyCode::Char('r'),
                    modifiers: KeyModifiers::NONE,
                    //clearing the screen and printing our message
                }) => {
                    execute!(stdout, Print(CodeCell::R.to_string())).unwrap();
                    guesses.push(CodeCell::R)
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('g'),
                    modifiers: KeyModifiers::NONE,
                }) => {
                    execute!(stdout, Print(CodeCell::G.to_string())).unwrap();
                    guesses.push(CodeCell::G)
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('b'),
                    modifiers: KeyModifiers::NONE,
                }) => {
                    execute!(stdout, Print(CodeCell::B.to_string())).unwrap();
                    guesses.push(CodeCell::B)
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('y'),
                    modifiers: KeyModifiers::NONE,
                }) => {
                    execute!(stdout, Print(CodeCell::Y.to_string())).unwrap();
                    guesses.push(CodeCell::Y)
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('m'),
                    modifiers: KeyModifiers::NONE,
                }) => {
                    execute!(stdout, Print(CodeCell::M.to_string())).unwrap();
                    guesses.push(CodeCell::M)
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers: KeyModifiers::NONE,
                }) => {
                    execute!(stdout, Print(CodeCell::C.to_string())).unwrap();
                    guesses.push(CodeCell::C)
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Backspace,
                    modifiers: KeyModifiers::NONE,
                }) => {
                    execute!(stdout, cursor::MoveLeft(1), Print(" "), cursor::MoveLeft(1)).unwrap();
                    if !guesses.is_empty() {
                        guesses.pop().unwrap();
                    }
                }
                //Event::Key(KeyEvent {
                //    code: KeyCode::Enter,
                //    modifiers: KeyModifiers::NONE,
                //}) => break,
                Event::Key(KeyEvent {
                    code: KeyCode::Char('x'),
                    modifiers: KeyModifiers::NONE,
                }) => return Err("User quit"),
                _ => (),
            }
        }
        let mut code = Code::new(self.peg_count as u8);
        code.0 = guesses;
        Ok(code)
    }
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Number of pegs
    #[clap(short, long, default_value_t = 4)]
    peg_count: u8,

    /// Number of guesses
    #[clap(short, long, default_value_t = 12)]
    guess_count: u8,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);

    let mut mm = MasterMind::new(args.peg_count);
    println!("{:?}", mm);
    println!("{}", mm.code);

    mm = MasterMind::new(args.peg_count);
    println!("{}", mm.code);

    mm = MasterMind::new(args.peg_count);
    println!("{}", mm.code);

    mm = MasterMind::new(args.peg_count);
    println!("{}", mm.code);

    println!();

    let mut guess = Code::new(args.peg_count);
    println!("{} {}", &guess, mm.guess(&guess));
    guess = Code::new(args.peg_count);
    println!("{} {}", &guess, mm.guess(&guess));
    guess = Code::new(args.peg_count);
    println!("{} {}", &guess, mm.guess(&guess));
    guess = Code::new(args.peg_count);
    println!("{} {}", &guess, mm.guess(&guess));

    println!();

    let mut stdout = stdout();

    enable_raw_mode().unwrap();
    //execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0), Print(r#"ctrl + q to exit, ctrl + h to print "Hello world", alt + t to print "crossterm is cool""#))
    //        .unwrap();
    execute!(
        stdout,
        Clear(ClearType::All),
        cursor::MoveTo(0, 0),
        Print(r#"Enter guesses <r,b,g,y,c,m>, or x to exit"#),
        cursor::MoveTo(0, 1),
    )
    .unwrap();

    let mut perfect_score = Score::new(mm.peg_count as u8);
    for n in 0..mm.peg_count {
        perfect_score.0[n] = Some(ScoreCell::K);
    }
    while let Ok(guess) = mm.get_guess(&mut stdout) {
        let score = mm.guess(&guess);
        println!(" {}", score);
        if score == perfect_score {
            break;
        };
    }

    disable_raw_mode().unwrap();
}
