use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn cli_runs_and_output_to_stdout() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("bank")?;

    cmd.arg("./tests/fixtures/regular_tx.csv")
        .assert()
        .success()
        .stdout(predicates::str::starts_with(
            "client,available,held,total,locked",
        ))
        .stdout(predicates::str::contains("1,1.5,0,1.5,false"))
        .stdout(predicates::str::contains("2,2.0,0,2.0,false"));

    Ok(())
}

#[test]
fn cli_fails_when_not_enough_arguments() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("bank")?;

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("not enough arguments"));

    Ok(())
}
