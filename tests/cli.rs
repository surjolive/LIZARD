use std::fs;
use std::process::{Command, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

fn temp_file(name: &str) -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("tests/data/{name}_{nanos}.lz")
}

fn lizard_binary() -> String {
    std::env::var("CARGO_BIN_EXE_lizard").expect("lizard test binary path")
}

#[test]
fn cli_version_and_help_are_available() {
    let version = Command::new(lizard_binary())
        .arg("--version")
        .output()
        .unwrap();
    assert!(version.status.success());
    assert!(String::from_utf8_lossy(&version.stdout).contains("LIZARD 0.1.0"));

    let help = Command::new(lizard_binary())
        .arg("--help")
        .output()
        .unwrap();
    assert!(help.status.success());
    assert!(String::from_utf8_lossy(&help.stdout).contains("build"));
}

#[test]
fn cli_evaluates_inline_source() {
    let output = Command::new(lizard_binary())
        .args(["-e", "say 10 + 20"])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "30\n");
}

#[test]
fn say_supports_function_style_syntax() {
    fs::create_dir_all("tests/data").unwrap();
    let path = temp_file("say_call");
    fs::write(&path, "hello = \"world\"\nsay(hello)\n").unwrap();

    let output = Command::new(lizard_binary()).arg(&path).output().unwrap();
    assert!(output.status.success());
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "world\n");
    let _ = fs::remove_file(path);
}

#[test]
fn repl_supports_help_and_version_flags() {
    let mut child = Command::new(lizard_binary())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    let mut stdin = child.stdin.take().unwrap();
    use std::io::Write;
    stdin.write_all(b"--help\nexit\n").unwrap();

    let output = child.wait_with_output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success());
    assert!(stdout.contains("LIZARD Programming Language"));
    assert!(!stdout.contains("Unknown variable: help"));
}

#[test]
fn single_quoted_strings_work_in_calls_and_inline_code() {
    fs::create_dir_all("tests/data").unwrap();
    let path = temp_file("single_quote");
    fs::write(&path, "say('hello world')\n").unwrap();

    let output = Command::new(lizard_binary()).arg(&path).output().unwrap();
    assert!(output.status.success());
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "hello world\n");

    let inline = Command::new(lizard_binary())
        .args(["-e", "say('hello world')"])
        .output()
        .unwrap();
    assert!(inline.status.success());
    assert_eq!(String::from_utf8(inline.stdout).unwrap(), "hello world\n");

    let _ = fs::remove_file(path);
}

#[test]
fn native_builtins_work_in_lizard_source() {
    fs::create_dir_all("tests/data").unwrap();
    let path = temp_file("builtins");
    fs::write(
        &path,
        "say size([1, 2, 3])\nsay typeOf(10)\nsay abs(-4)\nsay max(2, 9)\n",
    )
    .unwrap();
    let output = Command::new(lizard_binary())
        .args([&path])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        "3\nnumber\n4\n9\n"
    );
    let _ = fs::remove_file(path);
}

#[test]
fn extended_builtin_helpers_work() {
    fs::create_dir_all("tests/data").unwrap();
    let path = temp_file("extended_builtins");
    fs::write(
        &path,
        "say lower(\"LIZARD\")\nsay upper(\"lizard\")\nsay trim(\"  hi  \")\nsay contains(\"hello\", \"ell\")\nsay split(\"a-b-c\", \"-\")[1]\nsay join([\"a\", \"b\", \"c\"], \"-\")\nsay first([10, 20, 30])\nsay last([10, 20, 30])\nsay sum([1, 2, 3, 4])\nsay range(1, 4)[2]\n",
    )
    .unwrap();

    let output = Command::new(lizard_binary()).arg(&path).output().unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        "lizard\nLIZARD\nhi\ntrue\nb\na-b-c\n10\n30\n10\n3\n"
    );
    let _ = fs::remove_file(path);
}

#[test]
fn friendly_output_helpers_work() {
    fs::create_dir_all("tests/data").unwrap();
    let path = temp_file("friendly_helpers");
    fs::write(&path, "say aro(\"hi\")\nsay print(\"there\")\n").unwrap();

    let output = Command::new(lizard_binary()).arg(&path).output().unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "hi\nhi\nthere\nthere\n");
    let _ = fs::remove_file(path);
}

#[test]
fn feature_helpers_and_animation_work() {
    fs::create_dir_all("tests/data").unwrap();
    let path = temp_file("feature_helpers");
    fs::write(
        &path,
        "numbers = push([1, 2], 3)\nsay numbers[2]\nsay pop(numbers)\nsay reverse(\"lizard\")\nsay slice(\"lizard\", 1, 4)\nsay clamp(15, 0, 10)\nanimate(\"go\", 2, 0)\n",
    )
    .unwrap();

    let output = Command::new(lizard_binary()).arg(&path).output().unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        "3\n3\ndrazil\niza\n10\n\rgo 1/2\rgo 2/2\n"
    );
    let _ = fs::remove_file(path);
}

#[test]
fn cli_check_and_format_use_real_source() {
    fs::create_dir_all("tests/data").unwrap();
    let path = temp_file("cli");
    fs::write(&path, "if true\n say \"ok\"\nend\n").unwrap();

    let check = Command::new(lizard_binary())
        .args(["check", &path])
        .output()
        .unwrap();
    assert!(check.status.success());
    assert!(String::from_utf8_lossy(&check.stdout).contains("Checked"));

    let format = Command::new(lizard_binary())
        .args(["fmt", &path])
        .output()
        .unwrap();
    assert!(format.status.success());
    assert_eq!(
        String::from_utf8(format.stdout).unwrap(),
        "if true\n    say \"ok\"\nend\n"
    );

    let _ = fs::remove_file(path);
}

#[test]
fn build_creates_a_native_runtime_copy_and_adjacent_source() {
    fs::create_dir_all("tests/data").unwrap();
    let source_path = temp_file("build");
    let output_path = source_path.replace(".lz", ".exe");
    fs::write(&source_path, "say \"built\"\n").unwrap();

    let build = Command::new(lizard_binary())
        .args(["build", &source_path, "--output", &output_path])
        .output()
        .unwrap();
    assert!(
        build.status.success(),
        "build failed: {}",
        String::from_utf8_lossy(&build.stderr)
    );
    assert!(fs::metadata(&output_path).unwrap().len() > 0);

    let adjacent_source = output_path.replace(".exe", ".lz");
    assert!(fs::metadata(&adjacent_source).is_ok());
    let _ = fs::remove_file(source_path);
    let _ = fs::remove_file(output_path);
    let _ = fs::remove_file(adjacent_source);
}
