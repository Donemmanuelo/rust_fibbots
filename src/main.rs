mod tests;
use octocrab::Octocrab;
use regex::Regex;
use std::env;

use std::error::Error;


use tests::lib::fibonacci;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // GitHub API token and repository details
let github_token = env::var("TOKEN").map_err(|e| {
    eprintln!("Failed to fetch TOKEN: {:?}", e);
    e
})?;
    let repo_owner = env::var("GITHUB_REPOSITORY_OWNER").expect("GITHUB_REPOSITORY_OWNER not set");
    let repo_name = env::var("GITHUB_REPOSITORY_NAME").expect("GITHUB_REPOSITORY_NAME not set");
    let pull_number = env::var("GITHUB_PULL_REQUEST_NUMBER").expect("GITHUB_PULL_REQUEST_NUMBER not set");              
   let max_threshold = env::var("INPUT_MAX_THRESHOLD").expect("INPUT_MAX_THRESHOLD not set");      // Maximum number for Fibonacci computation
    let enable_fib = env::var("INPUT_ENABLE_FIB").expect("INPUT_ENABLE_FIB not set");      
    let max_threshold: u128 = max_threshold.trim().parse().expect("max_threshold not set");
    let enable_fib: bool = enable_fib.trim().parse().expect("enable_fib not set ");
    println!("GitHub Token: {}", github_token);


    let pull_number: u64 = pull_number.trim().parse().expect("pull_number not set");

    println!("Repository Owner: {}", repo_owner);
  
    println!("Pull Request Number: {}", pull_number);
     println!("enable fib: {}", enable_fib);
        println!("Repository name: {}", repo_name);

    println!("max threshold: {}", max_threshold);

    let octocrab = Octocrab::builder()
    .personal_token(github_token)
    .build()
    .map_err(|e| {
        eprintln!("Failed to initialize Octocrab: {:?}", e);
        e
    }).expect("could not connect");
println!("request made succesfully");
let user = octocrab.current().user().await
    .map_err(|e| {
        eprintln!("Failed to fetch user: {:?}", e);
        e
    })?;
println!("Authenticated User: {:?}", user.login);




//let user = octocrab.current().user().await?;
    // Fetch the pull request diff
    let pull_request = octocrab
        .pulls(&repo_owner, &repo_name)
        .get(pull_number)
        .await?;
 let diff_url = pull_request.diff_url.ok_or("No diff URL found")?;
println!("Diff URL: {:?}", diff_url);
    // Download the diff
    let diff_response = reqwest::get(diff_url).await?;
    let diff_text = diff_response.text().await?;

    // Extract numerical values from the diff
    let re = Regex::new(r"\b\d+\b")?;
    let mut numbers: Vec<u128> = re
        .find_iter(&diff_text)
        .filter_map(|m| m.as_str().parse().ok())
        .collect();

    // Fetch the pull request files
    let files = octocrab
        .pulls(&repo_owner, &repo_name)
        .list_files(pull_number)
        .await?;

    // Extract numerical values from all files
    for file in files {
        let file_url = file.raw_url.ok_or("No raw URL found for file")?;
        let file_response = reqwest::get(file_url).await?;
        let file_content = file_response.text().await?;

        // Extract numbers from file content
        let file_numbers: Vec<u128> = re
            .find_iter(&file_content)
            .filter_map(|m| m.as_str().parse().ok())
            .collect();
        numbers.extend(file_numbers);
       
    }
    println!("Extracted Numbers: {:?}", numbers);
    // Calculate Fibonacci for each number (if enabled)
    let mut comment_body = String::from("### Fibonacci Calculation Results\n");

    if enable_fib {
        for number in numbers {
            if number <= max_threshold {
                let fib = fibonacci(number);

               
                comment_body.push_str(&format!("- Number `{}`: Fibonacci = `{}`\n", number, fib));
            } else {
                comment_body.push_str(&format!("- Number `{}`: Exceeds max threshold (`{}`)\n", number, max_threshold));
            }
        }
        println!("comment body: {}", comment_body);

        octocrab
            .issues(repo_owner, repo_name)
            .create_comment(pull_number, comment_body)
            .await?;

        println!("Comment posted successfully!");
    } else {
        println!("Fibonacci computation and posting are disabled.");
    }

    Ok(())
}
