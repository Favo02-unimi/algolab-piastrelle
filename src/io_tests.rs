use std::io::Read;

#[cfg(test)]
use crate::*;

#[test]
fn test_small() {
    run_test(String::from("small"));
}

#[test]
fn test_blocco() {
    run_test(String::from("blocco"));
}

#[test]
fn test_propaga() {
    run_test(String::from("propaga"));
}

// --- utils functions ---

fn run_test(name: String) {
    run(Some(input(&name)), Some(test(&name)));
    assert!(is_same_file(output(&name), test(&name)).expect(""));
}

fn input(s: &str) -> String {
    format!("inputs/{}.in", s)
}

fn output(s: &str) -> String {
    format!("inputs/{}.out", s)
}

fn test(s: &str) -> String {
    format!("inputs/test_{}.out", s)
}

// https://users.rust-lang.org/t/efficient-way-of-checking-if-two-files-have-the-same-content/74735
fn is_same_file(file1: String, file2: String) -> Result<bool, std::io::Error> {
    let f1 = File::open(file1)?;
    let f2 = File::open(file2)?;

    // Check if file sizes are different
    if f1.metadata().unwrap().len() != f2.metadata().unwrap().len() {
        return Ok(false);
    }

    // Use buf readers since they are much faster
    let f1 = BufReader::new(f1);
    let f2 = BufReader::new(f2);

    // Do a byte to byte comparison of the two files
    for (b1, b2) in f1.bytes().zip(f2.bytes()) {
        if b1.unwrap() != b2.unwrap() {
            return Ok(false);
        }
    }

    Ok(true)
}
