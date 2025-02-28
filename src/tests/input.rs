pub fn parse_number(input: &str) -> Vec<f64> {
    input
    .split_whitespace()
    .map(|n| n.parse::<f64>().unwrap_or(0.0))
    .collect()
}