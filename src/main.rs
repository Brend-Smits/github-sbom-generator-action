use clap::Parser;

/// Retrieve GitHub SBOMs is a GitHub Actions composite action that retrieves Software Bill of Materials (SBOMs) for one or multiple repositories from GitHub's Dependency Graph API and saves them to a specified directory.
#[derive(Parser)]
struct Cli {
    #[clap(short, long)]
    // The path to a file containing a list of repository names to retrieve SBOMs for.
    repo_list_path: std::path::PathBuf,
    #[clap(short, long)]
    // The path to the directory where the retrieved SBOMs will be saved.
    save_directory_path: std::path::PathBuf,
    #[clap(short, long)]
    // The GitHub API token for authentication.
    token: String
}

fn main() {
    let args = Cli::parse();
    println!("{}", format!("Hello, world! `{}`, `{}`", &args.repo_list_path.display().to_string(), &args.save_directory_path.display().to_string()));
}
