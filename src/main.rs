use std::fs::File;
use std::io::{BufRead, BufReader};

use clap::Parser;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, USER_AGENT};
use serde_json::from_str;
use serde_json::Value;

/// Retrieve GitHub SBOMs is a GitHub Actions composite action that retrieves Software Bill of Materials (SBOMs) for one or multiple repositories from GitHub's Dependency Graph API and saves them to a specified directory.
#[derive(Parser)]
struct Cli {
    #[clap(short, long)]
    /// The path to a file containing a list of repository names to retrieve SBOMs for.
    repo_list_path: std::path::PathBuf,
    #[clap(short, long)]
    /// The path to the directory where the retrieved SBOMs will be saved.
    save_directory_path: std::path::PathBuf,
    #[clap(short, long)]
    /// The GitHub API token for authentication.
    token: Option<String>,
}

#[derive(Debug)]
pub struct CustomError(String);

fn main() -> Result<(), CustomError> {
    let args = Cli::parse();
    let client = reqwest::Client::new();
    read_file_line_by_line(args, &client, &fetch_sbom)?;
    Ok(())
}

// Function to read file line by line and process each line
fn read_file_line_by_line<F>(
    args: Cli,
    client: &reqwest::Client,
    process_line: &F,
) -> Result<(), CustomError>
where
    F: Fn(&str, &str, &reqwest::Client) -> Result<(), CustomError>,
{
    let file_path = &args.repo_list_path.display().to_string();
    let file = File::open(&file_path)
        .map_err(|err| CustomError(format!("Error reading `{}`: {}", file_path, err)))?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let content = match line {
            Ok(l) => l,
            Err(error) => {
                return Err(CustomError(format!(
                    "Error reading `{}`: {}",
                    file_path, error
                )));
            }
        };
        process_line(
            &args.token.as_ref().unwrap_or(&"".to_owned()),
            &content,
            client,
        )?;
    }
    Ok(())
}

#[tokio::main]
async fn fetch_sbom(
    token: &str,
    repo_name: &str,
    client: &reqwest::Client,
) -> Result<(), CustomError> {
    let api_url = format!(
        "https://api.github.com/repos/{}/dependency-graph/sbom",
        &repo_name
    );
    let mut headers = HeaderMap::new();
    headers.insert(
        ACCEPT,
        HeaderValue::from_static("application/vnd.github+json"),
    );
    headers.insert(
        "X-GitHub-Api-Version",
        HeaderValue::from_static("2022-11-28"),
    );
    if !token.is_empty() {
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", token)).expect("Expects bearer token"),
        );
    } else {
        println!("Token is not set! I can only access some public repositories. Consider using a token with --token option");
    }
    headers.insert(
        USER_AGENT,
        HeaderValue::from_static("sbom-downloader-action"),
    );
    // let client = reqwest::Client::new();
    let resp = client
        .get(&api_url)
        .headers(headers)
        .send()
        .await
        .expect("failed to get response");
    let response_text = resp.text().await.expect("failed to get payload");
    if response_text.contains("API rate limit exceeded") {
        println!("Error: API rate limit exceeded");
        std::process::exit(1);
    }
    if response_text.contains("Not Found") {
        println!("Error: Repository '{}' not found", repo_name);
        return Ok(());
    }
    if response_text.contains("Bad credentials") {
        println!("Error: Invalid Token, check token permissions and expiry date!");
        std::process::exit(1);
    }

    // Parse the response body as JSON into a SPDX struct
    let spdx: Value = match from_str(&response_text) {
        Ok(spdx) => spdx,
        Err(err) => {
            return Err(CustomError(format!(
                "Failed to parse JSON response: {}",
                err
            )));
        }
    };
    println!("{:#?}", spdx.to_string());
    Ok(())
}
