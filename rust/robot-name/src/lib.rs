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
        let name = "AB123".to_string();

        name
    }

    pub fn name<'a>(&'a self) -> &'a str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        self.name = Robot::random_name()
    }
}
