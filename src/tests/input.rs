
pub fn parse_number(input: &str) -> Vec<u128> {
    input
    .split_whitespace()
    .filter_map(|n| n.parse::<u128>().ok())
    .collect()
}