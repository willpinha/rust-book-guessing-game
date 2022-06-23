use std::io;
use std::cmp::Ordering;
use rand::Rng;

pub struct Game {
    random_number: u32
}

impl Game {
    pub fn new(min: u32, max: u32) -> Game {
        let random = Game::generate_random_number(min, max);
        Game { random_number: random }
    }

    fn generate_random_number(min: u32, max: u32) -> u32 {
        rand::thread_rng().gen_range(min..max)
    }

    pub fn validate_guess(&self, guess: &u32) -> bool {
        match guess.cmp(&self.random_number) {
            Ordering::Less => {
                println!("Too small!");
                false
            },
            Ordering::Greater => {
                println!("Too big!");
                false
            },
            Ordering::Equal => {
                println!("You win!");
                true
            }
        }
    }
}

pub struct User;

impl User {
    pub fn guess_number() -> u32 {
        println!("Please input your guess.");

        loop {
            let mut input = String::new();

            io::stdin().read_line(&mut input).expect("Failed to read line.");

            let guess: u32 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Not a number. Input your guess again.");
                    continue;
                }
            };

            return guess;
        }
    }
}
