extern crate rand;
use rand::{thread_rng, Rng, ThreadRng};

pub struct Robot {name: String}
impl Robot {
    pub fn new() -> Robot {
        // pick a random name
        Robot {name: Robot::random_name()}
    }

    /// Names should be 5 characters long, 2 letters followed by 3 numbers
    ///
    /// The provided test is broken, in that short names pass.
    fn random_name() -> String {
        let mut rng = Random(thread_rng());
        vec!(
            rng.letter(),
            rng.letter(),
            rng.digit(),
            rng.digit(),
            rng.digit()).into_iter().collect::<String>()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        self.name = Robot::random_name()
    }
}

struct Random(ThreadRng);
/// Inspired by `gen_ascii_chars` from `rand`
impl Random {
    pub fn letter(&mut self) -> char {
        const LETTERS : &'static [u8] =
                b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                  abcdefghijklmnopqrstuvwxyz";
        *self.0.choose(LETTERS).unwrap() as char
    }
    pub fn digit(&mut self) -> char {
        const DIGITS : &'static [u8] = b"0123456789";
        *self.0.choose(DIGITS).unwrap() as char
    }
}
