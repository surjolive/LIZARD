use std::fs;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn repeat_and_function_work() {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let path = format!("tests/data/functions_{nanos}.lz");
    fs::create_dir_all("tests/data").unwrap();
    fs::write(
        &path,
        "fn add(a, b)\n    return a + b\n\nrepeat 3\n    say add(10, 20)\n",
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
    assert_eq!(stdout, "30\n30\n30\n");

    let _ = fs::remove_file(path);
}
