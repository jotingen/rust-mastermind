use rand::distributions::{Distribution, Standard};
use rand::Rng;

#[derive(Debug, PartialEq)]
enum CodeCell {
    R, //Red
    G, //Green
    B, //Blue
    Y, //Yellow
    M, //Magenta
    C, //Cyan
}
#[derive(Debug)]
struct Code([CodeCell; 4]);
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
    pub fn new() -> Code {
        Code {
            0: [
                rand::random(),
                rand::random(),
                rand::random(),
                rand::random(),
            ],
        }
    }
}

#[derive(Debug)]
enum ResultCell {
    W, //White
    K, //Black
}
struct Result([Option<ResultCell>; 4]);
impl std::fmt::Display for ResultCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ResultCell::W => write!(f, "\x1b[37;47m \x1b[0m"),
            ResultCell::K => write!(f, "\x1b[91;101m \x1b[0m"),
        }
    }
}
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

#[derive(Debug)]
struct MasterMind {
    secret: Code,
}
impl MasterMind {
    pub fn new() -> MasterMind {
        MasterMind {
            secret: Code::new(),
        }
    }
    pub fn set_secret(&mut self, secret: Code) {
        self.secret = secret;
    }
    pub fn guess(&self, guess: &Code) -> Result {
        let mut result = Result {
            0: [None, None, None, None],
        };

        let mut used_code = [false, false, false, false];
        let mut used_guess = [false, false, false, false];
        let mut in_position = 0;
        let mut in_code = 0;

        //Scan for correct position
        for n in 0..4 {
            if guess.0[n] == self.secret.0[n] {
                used_code[n] = true;
                used_guess[n] = true;
                in_position += 1;
            }
        }
        //Scan for used in code, but not already used for position
        for n in 0..4 {
            if !used_guess[n] {
                for n_secret in 0..4 {
                    if !used_code[n] {
                        if guess.0[n] == self.secret.0[n_secret] {
                            used_code[n_secret] = true;
                            used_guess[n] = true;
                            in_code += 1;
                        }
                    }
                }
            }
        }

        //Build result
        for n in 0..4 {
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

fn main() {
    let mut mm = MasterMind::new();
    println!("{:?}", mm);
    println!("{}", mm.secret);

    mm = MasterMind::new();
    println!("{}", mm.secret);

    mm = MasterMind::new();
    println!("{}", mm.secret);

    mm = MasterMind::new();
    println!("{}", mm.secret);

    println!();

    let mut guess = Code::new();
    println!("{} {}", &guess, mm.guess(&guess));
    guess = Code::new();
    println!("{} {}", &guess, mm.guess(&guess));
    guess = Code::new();
    println!("{} {}", &guess, mm.guess(&guess));
    guess = Code::new();
    println!("{} {}", &guess, mm.guess(&guess));
    //mm.set_secret(Code{ cell: [Some(Cell::R), Some(Cell::G), Some(Cell::B), Some(Cell::Y)]});
    //println!("{}",mm.secret);
    //mm.set_secret(Code{ cell: [Some(Cell::K), Some(Cell::W), Some(Cell::K), Some(Cell::W)]});
    //println!("{}",mm.secret);
    //mm.set_secret(Code{ cell: [Some(Cell::R), Some(Cell::G), Some(Cell::B), Some(Cell::Y)]});
    //println!("{}",mm.guess(Code{ cell: [Some(Cell::R), Some(Cell::B), Some(Cell::R), None]}));

    println!("Hello, world!");
}
