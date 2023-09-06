use assert_cmd::prelude::*;
use predicates::prelude::*;
use serial_test::serial;
use std::{fs, path::PathBuf, process::Command};

#[test]
#[serial]
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

    let std_output = cmd
        .assert()
        .success()
        .stdout(predicate::str::contains(format!("Working in: {root}")))
        .stdout(predicate::str::contains("Assembling files: jul_21.yml"))
        .stdout(predicate::str::contains(
            "Validating from schema: book-schema.json",
        ))
        .stdout(predicate::str::contains("Assembling done!"))
        .stdout(predicate::str::contains("Outputing in: assembles"))
        .get_output()
        .clone();
    println!("{}", String::from_utf8_lossy(&std_output.stdout));

    //read assembles
    let assembled_file_path = PathBuf::from(&root).join(&output).join("jul_21.yml");
    let assembled_file = fs::read_to_string(assembled_file_path).unwrap();
    predicate::str::contains("title: Juliette coupe le gateau").eval(&assembled_file);

    //delete assembles folder
    fs::remove_dir_all(PathBuf::from(&root).join(&output)).unwrap();
}

#[test]
#[serial]
fn it_should_run_cli_with_relative_root() {
    let root = PathBuf::from("./tests/yml_test_files")
        .to_str()
        .unwrap()
        .to_string();

    let output = "assembles";

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("-r").arg(&root);
    cmd.arg("-f").arg("jul_21.yml");
    cmd.arg("-s").arg("book-schema.json");
    cmd.arg("-o").arg(output);

    let std_output = cmd.assert().success().get_output().clone();
    println!("{}", String::from_utf8_lossy(&std_output.stdout));

    //read assembles
    let assembled_file_path = PathBuf::from(&root).join(&output).join("jul_21.yml");
    let assembled_file = fs::read_to_string(assembled_file_path).unwrap();
    predicate::str::contains("title: Juliette coupe le gateau").eval(&assembled_file);

    //delete assembles folder
    fs::remove_dir_all(PathBuf::from(&root).join(&output)).unwrap();
}

#[test]
#[serial]
fn it_should_run_twice_and_completely_replace_file() {
    let root = PathBuf::from("./tests/yml_test_files")
        .to_str()
        .unwrap()
        .to_string();
    let output = "assembles";

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("-r").arg(&root);
    cmd.arg("-f").arg("jul_21.yml");
    cmd.arg("-s").arg("book-schema.json");
    cmd.arg("-o").arg(output);
    let std_output = cmd.assert().success().get_output().clone();
    println!("{}", String::from_utf8_lossy(&std_output.stdout));

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("-r").arg(&root);
    cmd.arg("-f").arg("jul_21.yml");
    cmd.arg("-s").arg("book-schema.json");
    cmd.arg("-o").arg(output);
    let std_output = cmd.assert().success().get_output().clone();
    println!("{}", String::from_utf8_lossy(&std_output.stdout));

    //read assembles
    let assembled_file_path = PathBuf::from(&root).join(&output).join("jul_21.yml");
    let assembled_file = fs::read_to_string(assembled_file_path).unwrap();
    let count = assembled_file
        .matches("title: Juliette coupe le gateau")
        .count();
    assert_eq!(count, 1);

    //delete assembles folder
    fs::remove_dir_all(PathBuf::from(&root).join(&output)).unwrap();
}

#[test]
#[serial]
fn it_should_fail() {
    let root = PathBuf::from("./non_existing_root")
        .to_str()
        .unwrap()
        .to_string();
    let output = "assembles";

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("-r").arg(&root);
    cmd.arg("-f").arg("jul_21.yml");
    cmd.arg("-s").arg("book-schema.json");
    cmd.arg("-o").arg(output);

    cmd.assert().failure();
}

#[test]
#[serial]
fn it_should_generate_file_with_ordered_entries() {
    let root = PathBuf::from("./tests/yml_test_files")
        .to_str()
        .unwrap()
        .to_string();

    let output = "assembles";

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("-r").arg(&root);
    cmd.arg("-f").arg("jul_21.yml");
    cmd.arg("-s").arg("book-schema.json");
    cmd.arg("-o").arg(output);

    let std_output = cmd.assert().success().get_output().clone();
    println!("{}", String::from_utf8_lossy(&std_output.stdout));

    //read assembles
    let assembled_file_path = PathBuf::from(&root).join(&output).join("jul_21.yml");
    let assembled_file = fs::read_to_string(assembled_file_path).unwrap();
    predicate::str::contains("title: Juliette coupe le gateau\nsummary: L'anniversaire de Juliette tourne mal\nstory:\ncontent: Ca y est ! Elle a 21 ans, et a invité tout le monde à pré coustille. Malheureusement Juliette n'est pas très adroite et se coupe le doigt en coupant le gâteau. Elle est emmenée à l'hôpital et se fait recoudre le doigt. Elle est très déçue de rater sa fête d'anniversaire.\nchapter: 5").eval(&assembled_file);

    //delete assembles folder
    fs::remove_dir_all(PathBuf::from(&root).join(&output)).unwrap();
}
