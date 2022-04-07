use clap::Parser;
use rand::distributions::{Distribution, Standard};
use rand::Rng;

//Mastermind

#[derive(Debug, PartialEq)]
enum CodeCell {
    R, //Red
    G, //Green
    B, //Blue
    Y, //Yellow
    M, //Magenta
    C, //Cyan
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
        Code {
            0: vector,
        }
    }
}

#[derive(Debug)]
enum ResultCell {
    W, //White
    K, //Black
}
impl std::fmt::Display for ResultCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ResultCell::W => write!(f, "\x1b[37;47m \x1b[0m"),
            ResultCell::K => write!(f, "\x1b[91;101m \x1b[0m"),
        }
    }
}
struct Result(Vec<Option<ResultCell>>);
impl std::fmt::Display for Result {
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
impl Result {
    pub fn new(peg_count: u8) -> Result {
        let mut vector: Vec<Option<ResultCell>> = Vec::new();
        for _ in 0..peg_count {
        vector.push(None);
        }
        Result {
            0: vector,
        }
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
    pub fn guess(&self, guess: &Code) -> Result {
        let mut result= Result::new(self.peg_count.try_into().unwrap());

        for _ in 0..self.peg_count {
            result.0.push(None);
        };

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
                    if !used_code[n] {
                        if guess.0[n] == self.code.0[n_code] {
                            used_code[n_code] = true;
                            used_guess[n] = true;
                            in_code += 1;
                        }
                    }
                }
            }
        }

        //Build result
        for n in 0..self.peg_count {
            if in_position > 0 {
                result.0[n] = Some(ResultCell::K);
                in_position -= 1;
            } else if in_code > 0 {
                result.0[n] = Some(ResultCell::W);
                in_code -= 1;
            }
        }

        result
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

}
