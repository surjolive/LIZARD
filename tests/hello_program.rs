use std::fs;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn hello_program_runs_and_prints_expected_output() {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let path = format!("tests/data/hello_{nanos}.lz");
    fs::create_dir_all("tests/data").unwrap();
    fs::write(
        &path,
        "say \"Hello World\"\n\nx = 10\ny = 20\n\nsay x + y\n\nif x < y\n    say \"LIZARD is simple and fast\"\n",
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
    assert_eq!(stdout, "Hello World\n30\nLIZARD is simple and fast\n");

    let _ = fs::remove_file(path);
}
