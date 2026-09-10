use std::fs;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn list_map_and_each_work() {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let path = format!("tests/data/collections_{nanos}.lz");
    fs::create_dir_all("tests/data").unwrap();
    fs::write(
        &path,
        "numbers = [10, 20, 30]\nitems = {\"name\": \"Surjo\", \"age\": 20}\n
say numbers[1]
say items[\"name\"]

each n in numbers
    say n
end
",
    )
    .unwrap();

    let binary = std::env::var("CARGO_BIN_EXE_lizard").expect("lizard test binary path");
    let output = Command::new(binary)
        .arg(&path)
        .output()
        .expect("failed to run lizard binary");

    assert!(
        output.status.success(),
        "lizard execution failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout, "20\nSurjo\n10\n20\n30\n");

    let _ = fs::remove_file(path);
}
