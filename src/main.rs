
use reqwest::Client;
mod tests;
use tests::lib::fibonacci;
use serde::Serialize;
use std::env;
#[derive(Debug)]
#[derive(Serialize)]
struct Comment {
    body: String,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parameters
    let max_threshold: u128 = 200; // Maximum number for Fibonacci computation
    let enable_fib: bool = true; // Enable/disable Fibonacci computation and comment posting

    // GitHub API URL for fetching the pull request diff
    let repo_owner = "donemmanuelo"; // Replace with the repository owner
    let repo_name = "rust_fibbots"; // Replace with the repository name
    let pull_number = 1; // Replace with the pull request number
    let diff_url = format!(
        "https://api.github.com/repos/{}/{}/pulls/{}/files",
        repo_owner, repo_name, pull_number
    );

    // GitHub personal access token for authentication
    let token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN environment variable not set");

    // Fetch the pull request diff
    let client = Client::new();
    let diff_response = client
        .get(&diff_url)
        .header("Authorization", format!("Bearer {}", token))
        .header("User-Agent", "reqwest")
        .header("Accept", "application/vnd.github.v3+json")
        .send()
        .await?;

    if !diff_response.status().is_success() {
        println!("Failed to fetch diff. Status: {}", diff_response.status());
        let error_message = diff_response.text().await?;
        println!("Error: {}", error_message);
        return Ok(());
    }

    // Parse the diff to extract numerical values
    let diff_text = diff_response.text().await?;
    let numbers: Vec<u128> = diff_text
        .split_whitespace()
        .filter_map(|word| word.parse::<u128>().ok())
        .filter(|&n| n <= max_threshold) // Filter numbers within the max_threshold
        .collect();

    // Check if Fibonacci computation and comment posting are enabled
    if !enable_fib {
        println!("Fibonacci computation and comment posting are disabled.");
        return Ok(());
    }

    // Calculate Fibonacci for each number
    let mut results = Vec::new();
    for number in numbers {
        let fib = fibonacci(number);
        results.push((number, fib));
    }

    // Format the results
    let comment_body = if results.is_empty() {
        format!("No numerical values found in the diff (max_threshold = {}).", max_threshold)
    } else {
        let mut output = format!("### Fibonacci Calculation Results (max_threshold = {})\n", max_threshold);
        for (number, fib) in results {
            output.push_str(&format!("- Position `{}`: Fibonacci = `{}`\n", number, fib));
        }
        output
    };


    // Post the comment to the pull request
    let comment_url = format!(
        "https://api.github.com/repos/{}/{}/issues/{}/comments",
        repo_owner, repo_name, pull_number
    );

    let comment = Comment { body: comment_body };
    let response = client
        .post(&comment_url)
        .header("Authorization", format!("Bearer {}", token))
        .header("User-Agent", "reqwest")
        .header("Accept", "application/vnd.github.v3+json")
        .json(&comment)
        .send()
        .await?;

    // Check the response status
    if response.status().is_success() {
        println!("Comment posted successfully!");
    } else {
        println!("Failed to post comment. Status: {}", response.status());
        let error_message = response.text().await?;
        println!("Error: {}", error_message);
    }
   // println!("{:?}", comment);

    Ok(())
}
