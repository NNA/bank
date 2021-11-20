use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn cli_runs_and_output_to_stdout() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("bank")?;

    cmd.arg("./tests/fixtures/regular_tx.csv")
        .assert()
        .success()
        .stdout(
            r#"client,available,held,total,locked
2,2.0,0,balance.total,false
1,1.5,0,balance.total,false
"#,
        );

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
