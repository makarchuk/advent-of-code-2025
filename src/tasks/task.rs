pub trait Task {
    type TaskInput: TaskInput;

    fn part1(&self, input: Self::TaskInput) -> String;
    fn part2(&self, input: Self::TaskInput) -> String;

    fn run(&self, input_str: &str, part: u32) -> String {
        let input = Self::TaskInput::from_str(input_str);
        match part {
            1 => self.part1(input),
            2 => self.part2(input),
            _ => panic!("Invalid part"),
        }
    }
}

pub trait TaskInput {
    fn from_str(s: &str) -> Self;
}
