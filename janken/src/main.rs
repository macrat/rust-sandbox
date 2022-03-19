use std::fmt;
use std::io;
use std::io::Write;

fn input_text() -> Result<String, io::Error> {
    let mut text = String::new();
    match io::stdin().read_line(&mut text) {
        Ok(_) => Ok(text),
        Err(err) => Err(err),
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    fn input() -> Hand {
        loop {
            print!("please input your hand (r, p, s) ");
            let _ = io::stdout().flush();

            match input_text() {
                Ok(text) => {
                    return match &*text {
                        "r\n" => Hand::Rock,
                        "p\n" => Hand::Paper,
                        "s\n" => Hand::Scissors,
                        _ => continue,
                    }
                }
                _ => continue,
            }
        }
    }

    fn rand() -> Hand {
        match rand::random::<u8>() % 3 {
            0 => Hand::Rock,
            1 => Hand::Paper,
            _ => Hand::Scissors,
        }
    }

    fn win(&self, another: Hand) -> bool {
        match (self, another) {
            (Hand::Rock, Hand::Paper) => false,
            (Hand::Paper, Hand::Scissors) => false,
            (Hand::Scissors, Hand::Rock) => false,
            _ if *self == another => false,
            _ => true,
        }
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Hand::Rock => write!(f, "rock"),
            Hand::Paper => write!(f, "paper"),
            Hand::Scissors => write!(f, "scissors"),
        }
    }
}

fn main() {
    let user = Hand::input();
    println!("You choose ... {}", user);

    let com = Hand::rand();
    println!("I choose ... {}", com);

    println!();

    if user == com {
        println!("  Draw!");
        println!();
        main();
    } else if user.win(com) {
        println!("  You win!");
        println!();
    } else {
        println!("  I win!");
        println!();
    }
}
