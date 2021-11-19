// TODO: Avoid using binary named "bank", make it work 'cargo run'
#[test]
fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("bank")?;

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("not enough arguments"));

    Ok(())
}
