use std::collections::{HashMap, HashSet};

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

    fn part_three(&self, input: &str) -> String {
        let racetrack = parse_racetrack(
            r#"S+= +=-== +=++=     =+=+=--=    =-= ++=     +=-  =+=++=-+==+ =++=-=-=--
- + +   + =   =     =      =   == = - -     - =  =         =-=        -
= + + +-- =-= ==-==-= --++ +  == == = +     - =  =    ==++=    =++=-=++
+ + + =     +         =  + + == == ++ =     = =  ==   =   = =++=       
= = + + +== +==     =++ == =+=  =  +  +==-=++ =   =++ --= + =          
+ ==- = + =   = =+= =   =       ++--          +     =   = = =--= ==++==
=     ==- ==+-- = = = ++= +=--      ==+ ==--= +--+=-= ==- ==   =+=    =
-               = = = =   +  +  ==+ = = +   =        ++    =          -
-               = + + =   +  -  = + = = +   =        +     =          -
--==++++==+=+++-= =-= =-+-=  =+-= =-= =--   +=++=+++==     -=+=++==+++-"#,
        );
        let actions = parse(input).values().next().cloned().expect("Empty input");
        let base = ranking_with_racetrack(&actions, &racetrack);
        total_winning_plans(&racetrack, base).to_string()
    }
}

fn total_winning_plans(racetrack: &[Action], base: usize) -> usize {
    fn dfs(
        racetrack: &[Action],
        base: usize,
        pluses: usize,
        minuses: usize,
        equals: usize,
        acc: &mut Vec<Action>,
        output: &mut HashSet<String>,
    ) {
        if pluses == 0 && minuses == 0 && equals == 0 {
            let score = ranking_with_racetrack(acc, racetrack);
            if score > base {
                let val = acc
                    .iter()
                    .map(|x| match x {
                        Action::Dec => '-',
                        Action::Inc => '+',
                        _ => '=',
                    })
                    .collect::<String>();
                output.insert(val);
            }
            return;
        }
        if pluses > 0 {
            acc.push(Action::Inc);
            dfs(racetrack, base, pluses - 1, minuses, equals, acc, output);
            acc.pop();
        }

        if minuses > 0 {
            acc.push(Action::Dec);
            dfs(racetrack, base, pluses, minuses - 1, equals, acc, output);
            acc.pop();
        }

        if equals > 0 {
            acc.push(Action::Keep);
            dfs(racetrack, base, pluses, minuses, equals - 1, acc, output);
            acc.pop();
        }
    }

    let mut output = HashSet::new();
    dfs(racetrack, base, 5, 3, 3, &mut Vec::new(), &mut output);
    output.len()
}

fn perform(data: HashMap<String, Vec<Action>>, segments: usize) -> String {
    let mut scores = HashMap::<String, Vec<usize>>::new();
    for segment in 0..segments {
        for key in data.keys() {
            let params = data.get(key).expect("Unreachable");
            let action = params[segment % params.len()];
            let scores = scores.entry(key.clone()).or_default();
            let mut score = *(scores.last().unwrap_or(&10));
            score = updated_score(score, action);
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
    let mut scores = HashMap::<String, Vec<usize>>::new();
    for round in 0..rounds * racetrack.len() {
        for key in data.keys() {
            let params = data.get(key).expect("Unreachable");
            let action = params[round % params.len()];
            let overridden = racetrack[round % racetrack.len()];
            let scores = scores.entry(key.clone()).or_default();
            let mut score = *(scores.last().unwrap_or(&10));
            score = updated_score(score, merged_action(overridden, action));
            scores.push(score);
        }
    }
    calc_ranking(&scores)
}

fn ranking_with_racetrack(params: &[Action], racetrack: &[Action]) -> usize {
    let mut scores = Vec::<usize>::new();
    for round in 0..2024 * racetrack.len() {
        let action = params[round % params.len()];
        let overridden = racetrack[round % racetrack.len()];
        let mut score = *(scores.last().unwrap_or(&10));
        score = updated_score(score, merged_action(overridden, action));
        scores.push(score);
    }
    scores.iter().sum()
}

fn merged_action(overridden: Action, action: Action) -> Action {
    match overridden {
        Action::Keep => action,
        _ => overridden,
    }
}

fn updated_score(score: usize, action: Action) -> usize {
    match action {
        Action::Dec => score.saturating_sub(1),
        Action::Inc => score + 1,
        _ => score,
    }
}

fn calc_ranking(scores: &HashMap<String, Vec<usize>>) -> String {
    let mut scores = scores
        .iter()
        .map(|(key, scores)| {
            let sum = scores.iter().sum::<usize>();
            (key, sum)
        })
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
        .split('\n')
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut visited = HashSet::new();
    let mut output = Vec::new();

    let rows = matrix.len();
    let (mut r, mut c) = (0, 0);
    loop {
        visited.insert((r, c));
        let cols = matrix[r].len();
        let Some(next) = Direction::all_clockwise()
            .iter()
            .map(|dir| match dir {
                Direction::Right if c < cols - 1 => (r, c + 1),
                Direction::Down if r < rows - 1 => (r + 1, c),
                Direction::Left if c > 0 => (r, c - 1),
                Direction::Up if r > 0 => (r - 1, c),
                _ => (r, c),
            })
            .filter(|(row, col)| !matrix[*row][*col].is_whitespace())
            .find(|point| !visited.contains(point))
        else {
            break;
        };
        (r, c) = next;
        let action = parse_action(matrix[r][c]);
        output.push(action);
    }
    output.push(Action::Keep);
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
