mod input;
use std::collections::HashSet;

use input::*;

pub struct Machine {
    desired_indicators: Vec<Indicator>,
    buttons: Vec<Vec<usize>>,
    joltage_requirements: Vec<usize>,
}

pub struct Task {}

impl crate::tasks::task::Task for Task {
    type TaskInput = Input;

    fn part1(&self, input: Self::TaskInput) -> String {
        input
            .machines
            .iter()
            .map(|input_machine| {
                let machine = Machine {
                    desired_indicators: input_machine.indicators.clone(),
                    buttons: input_machine.buttons.clone(),
                    joltage_requirements: input_machine.joltage_requirements.clone(),
                };

                machine.get_button_presses()
            })
            .sum::<usize>()
            .to_string()
    }

    fn part2(&self, input: Self::TaskInput) -> String {
        unimplemented!()
    }
}

impl Machine {
    fn get_button_presses(&self) -> usize {
        let current_state: Vec<Indicator> = vec![Indicator::Off; self.desired_indicators.len()];

        let mut visited_states: HashSet<Vec<Indicator>> = HashSet::new();

        let mut surface = HashSet::new();
        surface.insert(current_state);

        for i in 1.. {
            let mut next_surface = HashSet::new();

            for state in surface.iter() {
                for button in self.buttons.iter() {
                    let new_state = self.press_button(button, state);
                    if new_state == self.desired_indicators {
                        return i;
                    }

                    if !visited_states.contains(&new_state) {
                        next_surface.insert(new_state.clone());
                        visited_states.insert(new_state);
                    }
                }
            }

            visited_states.extend(surface.into_iter());
            surface = next_surface;
        }

        unreachable!()
    }

    fn press_button(&self, button: &Vec<usize>, current_state: &Vec<Indicator>) -> Vec<Indicator> {
        let mut new_state = current_state.clone();
        for &indicator_index in button {
            new_state[indicator_index] = match new_state[indicator_index] {
                Indicator::On => Indicator::Off,
                Indicator::Off => Indicator::On,
            };
        }
        new_state
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tasks::task::Task;
    use crate::tasks::task::TaskInput;

    const TEST_INPUT: &str = r#"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"#;

    #[test]
    fn test_part1() {
        let task = super::Task {};
        let input = Input::from_str(TEST_INPUT);
        let result = task.part1(input);
        assert_eq!(result, "7");
    }
}
