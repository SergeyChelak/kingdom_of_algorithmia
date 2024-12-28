pub fn reversed_str(s: &str) -> String {
    s.chars().rev().collect()
}

// fn split_into_trimmed_strings(input: &str) -> Vec<String> {
//     input
//         .split('\n')
//         .map(|s| s.trim())
//         .map(|s| s.to_string())
//         .collect::<Vec<String>>()
// }

pub trait TrimmedSplit {
    fn trimmed_split(&self) -> impl Iterator<Item = &str>;
}

impl TrimmedSplit for str {
    fn trimmed_split(&self) -> impl Iterator<Item = &str> {
        self.split('\n').map(|s| s.trim()).filter(|s| !s.is_empty())
    }
}
