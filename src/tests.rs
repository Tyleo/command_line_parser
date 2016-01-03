mod test_command_line {
    command_line!(
        TestCommandLine {
            (should_output_text, bool, "-t"),
            (bytes, String, "-b")
        }
    );
}

use tests::test_command_line::TestCommandLine;

#[test]
fn test_bool_parse() {
    let command_line = vec!["test_program_name.exe".to_string(),
                            "-t".to_string(),
                            "true".to_string(),
                            "-b".to_string(),
                            "0123456789ABCDEF".to_string()];
    let test_command_line = TestCommandLine::parse(command_line);

    let should_output_text = match test_command_line.should_output_text {
        Some(value) => {
            value
        },
        None => {
            false
        },
    };

    let bytes = match test_command_line.bytes {
        Some(value) => {
            value
        },
        None => {
            "Fail".to_string()
        },
    };

    assert!(should_output_text);
    assert!(bytes == "0123456789ABCDEF");
}
