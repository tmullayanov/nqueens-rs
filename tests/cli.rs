use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

const BIN: &str = "nqueens";

#[test]
fn zero_board_is_not_supported() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(BIN)?;

    cmd.arg("-n").arg("0");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("0 is not in"));

    Ok(())
}

#[test]
fn boards_larger_than_14_not_supported() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(BIN)?;

    cmd.arg("-n").arg("15");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("15 is not in"));

    Ok(())
}

#[test]
fn solves_sample_board_from_cli() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(BIN)?;

    let au = std::fs::read_to_string("tests/6x6.AU")
        .expect("tests/6x6.AU was not found or couldn't be read");

    cmd.arg("-n").arg("6");
    cmd.assert().success().stdout(predicate::str::contains(au));

    Ok(())
}

#[test]
fn solves_sample_board_prints_single_solution() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(BIN)?;

    let au = std::fs::read_to_string("tests/6x6.show_first.AU")
        .expect("tests/6x6.show_first.AU was not found or couldn't be read");

    cmd.arg("-n").arg("6").arg("--first-only");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(au).and(predicate::str::contains("Board: 2").not()));

    Ok(())
}

#[test]
fn doesnt_print_boards_on_zero_solution() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(BIN)?;

    let au = "Got 0 boards";

    cmd.arg("-n").arg("3").arg("--first-only");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(au).and(predicate::str::contains("Board: 1").not()));

    Ok(())
}
