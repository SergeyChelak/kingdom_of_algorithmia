pub fn reversed_str(s: &str) -> String {
    s.chars().rev().collect()
}

pub fn split_into_trimmed_strings(input: &str) -> Vec<String> {
    input
        .split('\n')
        .map(|s| s.trim())
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
}
