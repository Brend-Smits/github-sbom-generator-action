# GitHub SBOM Generator Contributing Guidelines

This application generates a Software Bill of Materials (SBOM) for a given GitHub repository/repositories, using the GitHub Dependency Graph API.
The Software Bill of Materials is generated based on the SPDX specification and is saved in JSON format.

## How to Run

1. Install Rust and Cargo: If you don't have Rust and Cargo installed, you can install them from the official Rust website at [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

2. Clone the Repository: Clone the SBOM generator repository to your local machine using the following command:

    ```BASH
    git clone git@github.com:Brend-Smits/retrieve-github-sbom-action.git
    ```

3. Build the Application: Navigate to the cloned repository directory and build the application using Cargo with the following command:

    ```BASH
    cd gitHub-sbom-generator
    cargo build
    ```

4. Run the Application: After the build is successful, you can run the SBOM generator application with the following command:

    ```BASH
    cargo run -- --input <path_to_input_file> --output <path_to_output_directory>
    ```

Replace `<path_to_input_file>` with the path to the file containing the list of GitHub repositories to generate SBOMs for, and `<path_to_output_directory>` with the path to the directory where the SBOMs will be saved.

5. View Generated SBOMs: The SBOMs will be generated in SPDX JSON format and saved in the specified output directory. You can view the generated SBOMs in the respective files with `.json` extension.

## How to Develop

1. Clone the Repository: Follow step 2 from the "How to Run" section to clone the GitHub SBOM Generator repository to your local machine.

2. Modify the Code: Open the cloned repository in your preferred Rust code editor and make the necessary changes.

3. Build and Test: Follow step 3 from the "How to Run" section to build and run the application locally. You can also write unit tests using Rust's built-in testing framework and run them using `cargo test` command.

4. Commit and Push Changes: After making changes, commit and push them to your forked repository.

5. Create Pull Request: If you want to contribute your changes to the original repository, create a pull request with your changes for review.

## How to Test

The GitHub SBOM Generator application comes with a built-in unit testing framework that you can use to run tests. Follow the steps below to run tests:

1. Navigate to the cloned repository directory in your terminal.

2. Run Tests: Use the following command to run all the tests:

    ```BASH
    cargo test
    ```

3. View Test Results: The test results will be displayed in the terminal, indicating whether the tests passed or failed.

4. Write Additional Tests: If you want to add more tests, you can write them in the `tests` directory in the repository and run them using the `cargo test` command.

That's it! You're now ready to run, develop, and test the GitHub SBOM Generator application. Happy coding!
