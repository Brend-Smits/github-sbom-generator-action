# GitHub SBOM(s) Generator Action

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

This GitHub Action and/or standalone CLI application generates a Software Bill of Materials (SBOM) for a given GitHub repository or a list of repositories, using the GitHub Dependency Graph API.
The Software Bill of Materials that is generated based on the SPDX specification and is saved in JSON format.
The GitHub Action is a composite action that you can easily integrate in your existing workflows.

## Usage

There are two ways to use this repository. One way is to use it as a composite GitHub action, the other is to use it as a standalone commandline application.
Both ways are explained below.

### Usage GitHub Action

To use this composite action in your workflow, you can include the following step in your workflow YAML file:

```yaml
- name: Generate GitHub SBOMs
  uses: brend-smits/github-sbom-generator-action@<tag-or-branch>
  with:
    repo_list_path: <path-to-repo-list-file>
    save_directory_path: <path-to-save-directory>
    github_token: ${{ secrets.GITHUB_TOKEN }}
```

Replace `<tag-or-branch>` with the specific tag or branch of the composite action that you want to use, `<path-to-repo-list-file>` with the path to a file containing a list of repository names, and `<path-to-save-directory>` with the path to the directory where you want to save the retrieved SBOMs. Also, make sure to provide the `GITHUB_TOKEN` secret in your workflow to authenticate with the GitHub API.

The composite action requires the following input:

- `repoListPath`: The path to a file containing a list of repository names to retrieve SBOMs for.
- `saveDirectoryPath`: The path to the directory where the retrieved SBOMs will be saved.
- `token`: The GitHub API token for authentication.

The composite action produces no outputs.

#### Example

Here's an example of how you can use the composite action in your workflow:

```YAML
name: Retrieve SBOMs

on:
  push:
    branches:
      - main

jobs:
  retrieve-sboms:
    runs-on: ubuntu-latest

    steps:
    - name: Checkout code
      uses: actions/checkout@v3

    - name: Retrieve GitHub SBOMs
      uses: brend-smits/retrieve-github-sbom-action@v0.1.0
      with:
        repo_list_path: gh-repos.txt
        save_directory_path: sboms
        github_token: ${{ secrets.GITHUB_TOKEN }}

      - name: Upload all sboms
        uses: actions/upload-artifact@0b7f8abb1508181956e8e162db84b466c27e18ce
        with:
          name: all-repos-sboms
          path: sboms
```

This workflow retrieves SBOMs for repositories listed in the `gh-repos.txt` file and saves them to a `sboms` directory.

## Usage CLI Application

Please refer to [CONTRIBUTING.md](./CONTRIBUTING.md) on how to build and run the application locally. There is currently no binary that is distributed yet.

## Usecases

Having the ability to retrieve (all) SBOM(s) from an organization in e.g. a central repository allows you to do all sorts of fun things like:
    - License checking
    - Metrics
    - Security checks
    - Compliancy checks with [Continuous Compliance](https://github.com/philips-labs/continuous-compliance-action)

## State

> :warning: Experimental!

This application/action is new and still undergoing testing. Feel free to give it a try and give suggestions or feedback by creating a new GitHub Issue.
