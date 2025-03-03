mod tests;
use tests::lib::fibonacci;
use reqwest::blocking::Client;
use serde_json::json;
use std::env;

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
        for file in response {
            if let Some(patch) = file["patch"].as_str() {
                println!("Patch Content: {:?}", patch); // Debug: Print patch content
                for line in patch.lines() {
                    println!("Line: {:?}", line); // Debug: Print each line
                    for word in line.split_whitespace() {
                        println!("Word: {:?}", word); // Debug: Print each word
                        // Skip empty strings
                        if word.is_empty() {
                            println!("Skipping empty word"); // Debug: Log skipped empty words
                            continue;
                        }
                        // Parse only if the word is numeric
                        if word.chars().all(|c| c.is_ascii_digit()) {
                            match word.parse::<u128>() {
                                Ok(num) => numbers.push(num),
                                Err(e) => println!("Failed to parse '{}': {}", word, e),
                            }
                        } else {
                            println!("Skipping non-numeric word: {}", word); // Debug: Log skipped non-numeric words
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
