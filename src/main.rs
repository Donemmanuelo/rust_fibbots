mod tests;
use reqwest::blocking::Client;
use serde_json::json;
use std::env;
use tests::lib::fibonacci;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // GitHub API details
    let repo = env::var("GITHUB_REPOSITORY")?;
    let pr_number = env::var("PR_NUMBER")?;
    let token = env::var("GITHUB_TOKEN")?;
    let api_url = format!(
        "https://api.github.com/repos/{}/pulls/{}/files",
        repo, pr_number
    );

    // Fetch the diff files
    let client = Client::new();
    let response = client
        .get(&api_url)
        .header("Authorization", format!("Bearer {}", token))
        .header("User-Agent", "Rust-GitHub-API")
        .send()?
        .json::<Vec<serde_json::Value>>()?;
    let mut numbers = Vec::new();

    // Iterate over each file in the response
    for file in response {
        // Check if the "patch" field exists and is a string
        if let Some(patch) = file.get("patch").and_then(|v| v.as_str()) {
            // Iterate over each line in the patch
            for line in patch.lines() {
                // Collect all contiguous sequences of digits
                let mut current_number = String::new();
                for char in line.chars() {
                    if char.is_numeric() {
                        // If the character is a digit, add it to the current number
                        current_number.push(char);

                        // If the current number is not empty and a non-digit is encountered, parse it
                        if let Ok(num) = current_number.parse::<u128>() {
                            numbers.push(num);
                        }
                        // Reset the current number
                        current_number.clear();
                    }
                }
                // Handle the case where the line ends with a number
                if !current_number.is_empty() {
                    if let Ok(num) = current_number.parse::<u128>() {
                        numbers.push(num);
                    }
                }
            }
        }
    }
    

    let max_threshold: u128 = env::var("MAX_THRESHOLD")?.parse()?;
    let enable_fib: bool = env::var("ENABLE_FIB")?.parse()?;

    // Compute Fibonacci for each number
    if enable_fib {
        let mut comment = "Fibonacci of numbers in the diff:\n".to_string();
        for num in numbers {
            if num <= max_threshold {
                let fib = fibonacci(num);
                comment.push_str(&format!("Fibonacci({}) = {}\n", num, fib));
            }
        }

        // Post the comment to the PR
        let comment_url = format!(
            "https://api.github.com/repos/{}/issues/{}/comments",
            repo, pr_number
        );
        let _ = client
            .post(&comment_url)
            .header("Authorization", format!("Bearer {}", token))
            .header("User-Agent", "Rust-GitHub-API")
            .json(&json!({ "body": comment }))
            .send()?;
    }
    Ok(())
}
