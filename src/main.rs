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

    // Extract numerical values from the diff
    let mut numbers = Vec::new();

    // Iterate over each file in the response
    for file in response {
        // Check if the "patch" field exists and is a string
        if let Some(patch) = file.get("patch").and_then(|v| v.as_str()) {
            // Iterate over each line in the patch
            for line in patch.lines() {
                // Split the line into words and iterate over each word
                for word in line.split_whitespace() {
                    // Remove or handle special characters before parsing
                    //let cleaned_word = word.trim()
                    //.chars()
                    //.filter(|c| c.is_numeric() || *c == '+' || *c == '-') // Allow + and - for signed numbers
                    //.collect::<String>();
                    let word = word.to_owned();
                    let cleaned_word: String = word
                        .trim()
                        .chars()
                        .filter(|char| char.is_digit(10))
                        .collect();

                    // Try to parse the cleaned word as a u128
                    if cleaned_word != " " {
                        match cleaned_word.parse::<u128>() {
                            Ok(num) => {
                                // If parsing is successful, push the number to the vector
                                numbers.push(num);
                            }
                            Err(e) => {
                                println!("error found a white space {e}")
                                // Handle invalid numbers (e.g., log or ignore)
                                // You can add logging or other error handling here if needed
                            }
                        }
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
