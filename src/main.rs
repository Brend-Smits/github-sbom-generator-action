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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let repo_name = "brend-smits/retrieve-github-sbom-action";
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
        HeaderValue::from_str(&format!("Bearer {}", &args.token))?,
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