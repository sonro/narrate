use std::process::{Command, Output, Stdio};

const STATUS_TEST_BIN: &str = "./target/debug/status_test";

#[test]
fn status_output_to_stderr() {
    let output = test_bin(STATUS_TEST_BIN, &["hi", "world", "green"]);
    // don't test for color because `status()` only outputs it to a TTY
    // by piping stderr in `test_bin` we therefore remove the color output
    let expected = format!("{:>12} {}\n", "hi", "world");
    let actual = String::from_utf8_lossy(&output.stderr);
    assert_eq!(expected, actual);
}

fn test_bin(binary: &str, args: &[&str]) -> Output {
    Command::new(binary)
        .args(args)
        .stderr(Stdio::piped())
        .spawn()
        .unwrap_or_else(|err| panic!("Failed to execute binary for testing: {}. {}", binary, err))
        .wait_with_output()
        .unwrap_or_else(|err| {
            panic!(
                "Failed to wait for test binary to finish: {}. {}",
                binary, err
            )
        })
}
