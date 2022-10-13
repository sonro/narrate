use narrate::ExitCode;

struct DefaultExitCode;
struct CustomExitCode;

const CUSTOM_CODE: i32 = -1;

impl ExitCode for DefaultExitCode {}
impl ExitCode for CustomExitCode {
    fn exit_code(&self) -> i32 {
        CUSTOM_CODE
    }
}

#[test]
fn default() {
    assert_eq!(exitcode::SOFTWARE, DefaultExitCode.exit_code());
}

#[test]
fn custom() {
    assert_eq!(CUSTOM_CODE, CustomExitCode.exit_code());
}
