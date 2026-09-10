use std::fs;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn while_loop_and_boolean_operators_work() {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let path = format!("tests/data/while_and_boolean_{nanos}.lz");
    fs::create_dir_all("tests/data").unwrap();
    fs::write(
        &path,
        "count = 0\nwhile count < 3\n    say count\n    count = count + 1\nend\nsay not false\nsay true and false\nsay true or false\n",
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
    assert_eq!(stdout, "0\n1\n2\ntrue\nfalse\ntrue\n");

    let _ = fs::remove_file(path);
}
