use std::fs::File;
use std::io::{BufRead, BufReader};

use clap::Parser;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, USER_AGENT};

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
    token: String,
}

#[derive(Debug)]
pub struct CustomError(String);

fn main() -> Result<(), CustomError> {
    let args = Cli::parse();

    let path_string = &args.repo_list_path.display().to_string();

    let file = File::open(&args.repo_list_path)
        .map_err(|err| CustomError(format!("Error reading `{}`: {}", path_string, err)))?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let content = match line {
            Ok(l) => l,
            Err(error) => {
                return Err(CustomError(format!(
                    "Error reading `{}`: {}",
                    path_string, error
                )));
            }
        };
        fetch_sbom(&args.token, &content).unwrap();
    }
    Ok(())
}

#[tokio::main]
async fn fetch_sbom(token: &str, repo_name: &str) -> Result<(), Box<dyn std::error::Error>> {
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
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", token))?,
    );
    headers.insert(
        USER_AGENT,
        HeaderValue::from_static("sbom-downloader-action"),
    );
    let client = reqwest::Client::new();
    let resp = client
        .get(&api_url)
        .headers(headers)
        .send()
        .await
        .expect("failed to get response")
        .text()
        .await
        .expect("failed to get payload");
    println!("{:#?}", resp);
    Ok(())
}
