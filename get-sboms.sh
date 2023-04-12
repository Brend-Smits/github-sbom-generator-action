#! /bin/bash

while getopts f:r:t:d flag; do
    case "${flag}" in
    r)
        repoListPath=${OPTARG}
        echo "=== Repository List Path set to: $repoListPath ==="
        ;;
    t)
        token=${OPTARG}
        ;;
    d)
        echo "=== Debug Mode Enabled ==="
        set -x
        ;;
    f)
        saveDirectoryPath=${OPTARG}
        echo "=== Save Directory Path set to: $saveDirectoryPath ==="
        ;;
    *)
        echo "usage: $0 [-r] [-t] [-d] [-f]" >&2
        exit 1
        ;;
    esac
done

if [ -z "$repoListPath" ]; then
    echo "=== You did not specify a Repository List Path. Use -r parameter to specify one. ==="
    exit 1
fi

if [ -z "$saveDirectoryPath" ]; then
    echo "=== You did not specify a Save Directory Path. Use -f parameter to specify one. ==="
    exit 1
fi

if [ -z "$token" ]; then
    echo "=== You did not specify a GitHub Token. Use -t parameter to specify one. ==="
    exit 1
fi

# Check if repoListPath directory and file exist and are not empty
# if [ ! -d "$repoListPath" ] || [ ! -s "$repoListPath" ]; then
#     echo "=== Repository List Path does not exist or is empty. Exiting gracefully ==="
#     exit 0
# fi

# echo $repoListPath

# Function to call GitHub API and retrieve SBOM for a repository
get_sbom() {
  local repo_name=$1
  local save_directory=$2

  # Call GitHub API to retrieve SBOM
  local response=$(curl -s -L \
    -H "Accept: application/vnd.github+json" \
    -H "Authorization: Bearer ${token}" \
    -H "X-GitHub-Api-Version: 2022-11-28" \
    "https://api.github.com/repos/${repo_name}/dependency-graph/sbom")

  # Check for rate limit exceeded error
  if [[ $response == *"API rate limit exceeded"* ]]; then
    echo "Error: API rate limit exceeded"
    exit 1
  fi

  # Check for other errors
  if [[ $response == *"404 Not Found"* ]]; then
    echo "Error: Repository not found: ${repo_name}"
    return
  elif [[ $response == *"403 Forbidden"* ]]; then
    echo "Error: Forbidden: ${repo_name}"
    return
  fi

  # Create directory for SBOM if it doesn't exist
  if [[ ! -d "${save_directory}/${repo_name}" ]]; then
    mkdir -p "${save_directory}/${repo_name}"
  fi

  # Save SBOM to file
  echo "$response" >"${save_directory}/${repo_name}/sbom.json"

  echo "SBOM saved for repository: ${repo_name}"
}

# Function to process each repository in the list
process_repo_list() {
  local repo_list_path=$1
  local save_directory=$2

  # Loop over each line in the repo list file
  while IFS= read -r repo_name; do
    echo "Processing repository: ${repo_name}"
    get_sbom "${repo_name}" "${save_directory}"
  done <"${repo_list_path}"
}

# Call the function with provided variables
process_repo_list "${repoListPath}" "${saveDirectoryPath}"


