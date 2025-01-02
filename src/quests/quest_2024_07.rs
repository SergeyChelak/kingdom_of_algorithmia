use std::collections::HashMap;

use crate::{
    common::{strings::TrimmedSplit, Direction},
    quests::{Quest, QuestInputLoader, Solution},
};

pub fn assemble() -> Quest {
    Quest {
        title: "Quest 2024 7: Not Fast but Furious".to_string(),
        input_loader: QuestInputLoader::with_quest_date(2024, 7),
        solution: Box::new(Q2024_7),
    }
}

struct Q2024_7;

impl Solution for Q2024_7 {
    fn part_one(&self, input: &str) -> String {
        let data = parse(input);
        perform(data, 10)
    }

    fn part_two(&self, input: &str) -> String {
        let racetrack = parse_racetrack(
            "S-=++=-==++=++=-=+=-=+=+=--=-=++=-==++=-+=-=+=-=+=+=++=-+==++=++=-=-=--
-                                                                     -
=                                                                     =
+                                                                     +
=                                                                     +
+                                                                     =
=                                                                     =
-                                                                     -
--==++++==+=+++-=+=-=+=-+-=+-=+-=+=-=+=--=+++=++=+++==++==--=+=++==+++-",
        );
        let data = parse(input);
        perform_with_racetrack(data, 10, &racetrack)
    }

    fn part_three(&self, _input: &str) -> String {
        todo!()
    }
}

fn perform(data: HashMap<String, Vec<Action>>, segments: usize) -> String {
    let mut scores = HashMap::<String, Vec<usize>>::new();
    for segment in 0..segments {
        for key in data.keys() {
            let params = data.get(key).expect("Unreachable");
            let action = params[segment % params.len()];
            let scores = scores.entry(key.clone()).or_default();
            let mut score = *(scores.last().unwrap_or(&10));
            score = match action {
                Action::Dec => score.saturating_sub(1),
                Action::Inc => score + 1,
                _ => score,
            };
            scores.push(score);
        }
    }
    calc_ranking(&scores)
}

fn perform_with_racetrack(
    data: HashMap<String, Vec<Action>>,
    rounds: usize,
    racetrack: &[Action],
) -> String {
    let merged = |overridden: Action, action: Action| -> Action {
        match overridden {
            Action::Keep => action,
            _ => overridden,
        }
    };
    let mut scores = HashMap::<String, Vec<usize>>::new();
    for round in 0..rounds * racetrack.len() {
        for key in data.keys() {
            let params = data.get(key).expect("Unreachable");
            let action = params[round % params.len()];
            let overridden = racetrack[round % racetrack.len()];
            let scores = scores.entry(key.clone()).or_default();
            let mut score = *(scores.last().unwrap_or(&10));
            score = match merged(overridden, action) {
                Action::Dec => score.saturating_sub(1),
                Action::Inc => score + 1,
                _ => score,
            };
            scores.push(score);
        }
    }
    calc_ranking(&scores)
}

fn calc_ranking(scores: &HashMap<String, Vec<usize>>) -> String {
    let mut scores = scores
        .iter()
        .map(|(key, scores)| {
            let sum = scores.iter().sum::<usize>();
            (key, sum)
        })
        // .inspect(|x| println!("{:?}", x))
        .collect::<Vec<_>>();
    scores.sort_by(|a, b| b.1.cmp(&a.1));
    scores
        .iter()
        .map(|(val, _)| *val)
        .cloned()
        .collect::<Vec<_>>()
        .join("")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Action {
    Inc,
    Dec,
    Keep,
}

fn parse(input: &str) -> HashMap<String, Vec<Action>> {
    input
        .trimmed_split()
        .map(parse_line)
        .collect::<HashMap<_, _>>()
}

fn parse_line(input: &str) -> (String, Vec<Action>) {
    let (name, actions) = input.split_once(':').expect("Invalid input format");
    let actions = actions
        .split(',')
        .map(|action| parse_action(action.chars().next().expect("Invalid actions format")))
        .collect::<Vec<_>>();
    (name.to_string(), actions)
}

fn parse_action(ch: char) -> Action {
    match ch {
        '+' => Action::Inc,
        '-' => Action::Dec,
        _ => Action::Keep,
    }
}

fn parse_racetrack(input: &str) -> Vec<Action> {
    let matrix = input
        .trimmed_split()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut output = Vec::new();
    let mut dir = Some(Direction::Right);

    let next_dir = |d: Direction| -> Option<Direction> {
        use Direction::*;
        match d {
            Right => Some(Down),
            Down => Some(Left),
            Left => Some(Up),
            Up => None,
        }
    };

    let rows = matrix.len();
    let (mut r, mut c) = (0, 0);
    while dir.is_some() {
        let cols = matrix[r].len();
        match dir.unwrap() {
            Direction::Right if c < cols - 1 => c += 1,
            Direction::Down if r < rows - 1 => r += 1,
            Direction::Left if c > 0 => c -= 1,
            Direction::Up if r > 0 => r -= 1,
            _ => {
                dir = next_dir(dir.unwrap());
                continue;
            }
        }
        let action = parse_action(matrix[r][c]);
        output.push(action);
    }
    output
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn quest2024_07_part1() {
        let input = "A:+,-,=,=
B:+,=,-,+
C:=,-,+,+
D:=,=,=,+";
        assert_eq!("BDCA", Q2024_7.part_one(input))
    }

    #[test]
    fn quest2024_07_parse_track() {
        let input = "S+===
-   +
=+=-+";
        let track = parse_racetrack(input);
        let (_, line) = parse_line("X:+,=,=,=,+,+,-,=,+,=,-,S");
        println!("PARSED: {track:?}");
        println!("INPUT:  {line:?}");
        assert_eq!(track, line)
    }

    #[test]
    fn quest2024_07_part2() {
        let racetrack = parse_racetrack(
            "S+===
-   +
=+=-+",
        );
        let input = "A:+,-,=,=
B:+,=,-,+
C:=,-,+,+
D:=,=,=,+";
        let data = parse(input);
        let ranking = perform_with_racetrack(data, 10, &racetrack);
        assert_eq!(ranking, "DCBA")
    }
}
