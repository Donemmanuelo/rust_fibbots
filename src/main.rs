use std::env;



use reqwest::Client;
use serde::Serialize;
use regex::Regex;
use tests::lib;

// Struct for the comment body
#[derive(Serialize)]
struct CommentBody {
    body: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let max_threshold = env::var("INPUT_MAX_THRESHOLD").unwrap_or_else(|_| {
        eprintln!("Environment variable 'max_threshold' not set");
        std::process::exit(1);
    });
    let enable_fib = env::var("INPUT_ENABLE_FIB").unwrap_or_else(|_| {
        eprintln!("Environment variable 'enable_fib' not set");
        std::process::exit(1);
    });
    let v: u128 = max_threshold.trim().parse().expect("invalid input");
    let u: bool = enable_fib.trim().parse().expect("invalid input");

    // Get environment variables
    //let repo_owner = env::var("GITHUB_ENV").expect("GITHUB_REPOSITORY_OWNER not set");
    let repo_owner = env::var("GITHUB_REPOSITORY").expect("GITHUB_REPOSITORY_OWNER not set");
    let repo_name = env::var("GITHUB_REPOSITORY").expect("GITHUB_REPOSITORY_NAME not set");
    let pr_number = env::var("GITHUB_EVENT_NAME").expect("PR_NUMBER not set");
    let token = env::var("INPUT_GITHUB_TOKEN").expect("GITHUB_TOKEN not set");

    // Create a reqwest client
    let client = Client::new();

    // Fetch the pull request diff
    let diff_url = format!(
        "https://api.github.com/repos/{repo_owner}/{repo_name}/pulls/{pr_number}",
         
    );

   
    let diff_response = client
        .get(&diff_url)
        .header("Authorization", format!("Bearer {}", token))
        .header("User-Agent", "reqwest")
        .header("Accept", "application/vnd.github.v3.diff")
        .send()
        .await?;

    if !diff_response.status().is_success() {
        println!(
            "Failed to fetch pull request diff. Status: {}",
            diff_response.status()
        );
        println!("Response: {:?}", diff_response.text().await?);
        return Ok(());
    }

    let diff = diff_response.text().await?;

    // Regex to find numerical values in the diff
    let num_regex = Regex::new(r"\b\d+\b").unwrap();

    // Extract numerical values from the diff
    let numbers: Vec<u128> = num_regex
        .find_iter(&diff)
        .filter_map(|m| m.as_str().parse().ok())
        .collect();
  
    // Compute Fibonacci for each numerical value
    let mut computed_results = String::new();
    if !numbers.is_empty() {
        computed_results.push_str("### Fibonacci Results\n\n");
        computed_results.push_str("Here are the Fibonacci sequences for numerical values found in the diff:\n\n");
        computed_results.push_str("| Number | Fibonacci |\n|--------|------------|\n");
        for num in numbers {
            let fib = lib::fibbonnacci(num);
           if u == true && v >= num {
            computed_results.push_str(&format!("| {} | {} |\n", num, fib));}
        }
    } 

    // Create the comment body
    let comment_body = CommentBody {
        body: computed_results,
    };

    // Create the API URL for posting a comment
    let comment_url = format!(
        "https://api.github.com/repos/{}/{}/issues/{}/comments",
        repo_owner, repo_name, pr_number
    );

    // Make the POST request to post the comment
    let response = client
        .post(&comment_url)
        .header("Authorization", format!("Bearer {}", token))
        .header("User-Agent", "reqwest")
        .header("Accept", "application/vnd.github.v3+json")
        .json(&comment_body)
        .send()
        .await?;

    // Check the response status
    if response.status().is_success() {
        println!("Comment posted successfully!");
    } else {
        println!("Failed to post comment. Status: {}", response.status());
        println!("Response: {:?}", response.text().await?);
    }

    Ok(())
}
mod tests; /*

           #[cfg(test)]
           mod test {
               use crate::tests::lib::fibbonnacci;

               #[test]
               fn test_main() {
                   use crate::main;
                   assert_eq!(main(), ());
               }
               let  z: Vec<f64> = vec![8.0];

               #[test]
               fn test_fibbonacci() {
                   assert_ne!(fibbonnacci(100, false, z), 21);
                   assert_eq!(fibbonnacci(1, true, 1), 1);
               }
           }
           */
