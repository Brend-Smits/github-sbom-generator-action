use assert_cmd::prelude::*;
use assert_fs::prelude::FileWriteStr;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn retrieves_sbom_from_github() -> Result<(), Box<dyn std::error::Error>> {
    // Arrange
    let file = assert_fs::NamedTempFile::new("repos.txt")?;
    file.write_str("brend-smits/retrieve-github-sbom-action")?;

    // Act
    let mut cmd = Command::cargo_bin("retrieve-github-sbom")?;
    cmd.arg("--repo-list-path")
        .arg(file.path())
        .arg("--save-directory-path")
        .arg("target/tmp/");

    // Assert
    cmd.assert().success().stdout(predicate::str::contains(
        "com.github.Brend-Smits/retrieve-github-sbom-action",
    ));
    cmd.assert().failure().stdout(predicate::str::contains(
        "Token is not set! I can only access some public repositories. Consider using a token with --token option",
    ));
    Ok(())
}

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("retrieve-github-sbom")?;

    cmd.arg("--repo-list-path")
        .arg("test/file/doesnt/exist")
        .arg("--save-directory-path")
        .arg("test/file/doesnt/exist")
        .arg("--token")
        .arg("foo");
    cmd.assert().failure().stderr(predicate::str::contains(
        "Error reading `test/file/doesnt/exist`: No such file or directory (os error 2)",
    ));

    Ok(())
}

#[test]
fn invalid_token_should_error() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("brend-smits/retrieve-github-sbom-action\nbrend-smits-repo-doesnt-exist")?;

    let mut cmd = Command::cargo_bin("retrieve-github-sbom")?;
    cmd.arg("--repo-list-path")
        .arg(file.path())
        .arg("--save-directory-path")
        .arg("test/path/doesnt-exist")
        .arg("--token")
        .arg("foo");
    cmd.assert().failure().stdout(predicate::str::contains(
        "Error: Invalid Token, check token permissions and expiry date!\n",
    ));

    Ok(())
}

#[test]
fn non_existent_repo_should_log_and_continue() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str(
        "brend-smits/retrieve-github-sbom-action\nbrend-smits/repo-doesnt-exist\ngithub/licensed",
    )?;

    let mut cmd = Command::cargo_bin("retrieve-github-sbom")?;
    cmd.arg("--repo-list-path")
        .arg(file.path())
        .arg("--save-directory-path")
        .arg("target/tmp");
    cmd.assert().success().stdout(predicate::str::contains(
        "Repository 'brend-smits/repo-doesnt-exist' not found",
    ));

    Ok(())
}
