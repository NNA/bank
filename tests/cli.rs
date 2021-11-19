use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

// TODO: Avoid using binary named "bank", make it work 'cargo run'
#[test]
fn not_enough_arguments() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("bank")?;

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("not enough arguments"));

    Ok(())
}

// #[test]
// fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
//     let mut cmd = Command::cargo_bin("bank")?;

//     cmd.arg("test/file/doesnt/exist");
//     cmd.assert()
//         .failure()
//         .stderr(predicate::str::contains("No such file or directory"));

//     Ok(())
// }
