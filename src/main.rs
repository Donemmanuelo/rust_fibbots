use reqwest::blocking::Client;
use serde::Serialize;
use std::env;
use tests::input::extract_numbers;
use tests::lib::fibbonnacci;
use tests::value::bal;

use tests::ser::MyStruct;
#[derive(Serialize)]
struct Comment {
    body: String,
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let f: &str = &bal();
    let numbers = extract_numbers(f);
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

    let token = env::var("GITHUB_TOKEN").expect("TOKEN not set");

    let repository = env::var("GITHUB_REPOSITORY").expect("REPOSITORY not set");
    let pr_number = env::var("GITHUB_ENV").expect("PR_NUMBER not set");

    let url = format!(
        "https://api.github.com/repos/{}/issues/{}/comments",
        repository, pr_number
    );
    for i in 0..numbers.len() {
        if u == true && v >= numbers[i] {
            let x = fibbonnacci(v, u, numbers[i]);
            let t = MyStruct { value: x };
            println!("The fibbonnacci of {:?} is: {:?}", numbers[i], t.value);

            let comment_body = format!(
                "The Fibonacci number at position {} is **{}**.",
                numbers[i], t.value
            );
            let comment = Comment {
                body: comment_body.to_string(),
            };
    

    // let comment_body = "This is a comment posted from a Rust script running in a GitHub Actions workflow!";

    // Send the POST request
    let client = Client::new();
    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", token))
        .header("User-Agent", "Rust GitHub API Client")
        .json(&comment)
        .send()?;
        

    if response.status().is_success() {
        println!("Comment posted successfully!");
    } else {
        println!("Failed to post comment. Status: {}", response.status());
    }
        }
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
