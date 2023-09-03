use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::{fs, path::PathBuf, process::Command}; // Run programs

#[test]
fn it_should_run_cli() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/yml_test_files")
        .to_str()
        .unwrap()
        .to_string();

    let output = "assembles";

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("-r").arg(&root);
    cmd.arg("-f").arg("jul_21.yml");
    cmd.arg("-s").arg("book-schema.json");
    cmd.arg("-o").arg(output);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(format!("Working in: {root}")))
        .stdout(predicate::str::contains("Assembling files: jul_21.yml"))
        .stdout(predicate::str::contains(
            "Validating from schema: book-schema.json",
        ))
        .stdout(predicate::str::contains("Outputing in: assembles"));

    //read assembles
    let assembled_file_path = PathBuf::from(&root).join(&output).join("jul_21.yml");
    let assembled_file = fs::read_to_string(assembled_file_path).unwrap();
    predicate::str::contains("title: Juliette coupe le gateau").eval(&assembled_file);

    //delete assembles folder
    fs::remove_dir_all(PathBuf::from(&root).join(&output)).unwrap();
}
