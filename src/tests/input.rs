use regex::Regex;

pub fn extract_numbers(input: &str) -> Vec<u128> {
  
    let re = Regex::new(r"\d+").unwrap();
    
  
    re.find_iter(input)
        .map(|digits| digits.as_str().parse::<u128>().unwrap())
        .collect()
}
