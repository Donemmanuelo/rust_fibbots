/*use reqwest::blocking::Client;
use serde_json::json;
use std::env;
use num_bigint::BigInt;

pub fn post_comment_to_pr(owner: &str, repo: &str, pr_number: u64, body: &BigInt) -> Result<(), reqwest::Error> {
    // Get GitHub token from environment variable
    let github_token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not set");

    // GitHub API endpoint for posting a comment to a PR
    let url = format!("https://api.github.com/repos/{}/{}/issues/{}/comments", owner, repo, pr_number);

    // Create the HTTP client
    let client = Client::new();

    // Send the POST request
    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", github_token))
        .header("User-Agent", "Rust-GitHub-API-Client")
        .header("Accept", "application/vnd.github.v3+json")
        .json(&json!({ "body": body }))
        .send()?;

    // Check if the request was successful
    if response.status().is_success() {
        println!("Comment posted successfully!");
    } else {
        eprintln!("Failed to post comment: {}", response.status());
    }

    Ok(())
} */