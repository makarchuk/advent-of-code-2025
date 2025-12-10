pub struct Input {
    pub machines: Vec<MachineInput>,
}
pub struct MachineInput {
    pub indicators: Vec<Indicator>,
    pub buttons: Vec<Vec<usize>>,
    pub joltage_requirements: Vec<usize>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Indicator {
    On,
    Off,
}

impl crate::tasks::task::TaskInput for Input {
    fn from_str(s: &str) -> Self {
        let machines = s
            .lines()
            .map(|line| {
                let chars = line.chars().collect::<Vec<char>>();
                //[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
                assert!(chars[0] == '[');
                assert!(chars[chars.len() - 1] == '}');

                //.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7
                let chars: String = chars[1..chars.len() - 1].iter().collect();

                let (indicators, buttons, joltage_requirements) = {
                    let chunks = chars.split("] ").collect::<Vec<_>>();
                    assert!(chunks.len() == 2);

                    let rest_chunks = chunks[1].split(" {").collect::<Vec<_>>();
                    assert!(rest_chunks.len() == 2);

                    (chunks[0], rest_chunks[0], rest_chunks[1])
                };

                let indicators = indicators
                    .chars()
                    .map(|c| match c {
                        '#' => Indicator::On,
                        '.' => Indicator::Off,
                        _ => panic!("Invalid indicator character: {}", c),
                    })
                    .collect();

                let buttons = buttons
                    .split(" ")
                    .map(|b| {
                        b.trim_start_matches("(")
                            .trim_end_matches(")")
                            .split(",")
                            .map(|n| n.parse::<usize>().unwrap())
                            .collect::<Vec<usize>>()
                    })
                    .collect();

                let joltage_requirements = joltage_requirements
                    .split(",")
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();

                MachineInput {
                    indicators,
                    buttons,
                    joltage_requirements,
                }
            })
            .collect();

        Input { machines }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tasks::task::TaskInput;

    #[test]
    fn test_from_str() {
        let test_input = r#"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"#;
        let input = Input::from_str(test_input);

        assert_eq!(input.machines.len(), 3);
        assert_eq!(input.machines[0].indicators.len(), 4);
        assert_eq!(input.machines[0].buttons.len(), 6);
        assert_eq!(input.machines[0].joltage_requirements.len(), 4);

        assert_eq!(input.machines[1].buttons[4], vec![1, 2, 3, 4]);
        assert_eq!(
            input.machines[2].indicators,
            vec![
                Indicator::Off,
                Indicator::On,
                Indicator::On,
                Indicator::On,
                Indicator::Off,
                Indicator::On
            ]
        );
        assert_eq!(input.machines[0].joltage_requirements, vec![3, 5, 4, 7]);
    }
}
