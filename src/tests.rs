mod test_command_line {
    command_line!(
        TestCommandLine {
            (should_output_text, bool, "-t", "If true, the program will outpuut ASCII text; otherwise it will output hex text."),
            (bytes, String, "-b", "The two byte strings to xor together. These must be separated by a space."),
            (vec, Vec<String>, "-v", "Vec test.")
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
                            "0123456789ABCDEF".to_string(),
                            "-v".to_string(),
                            "3".to_string(),
                            "A".to_string(),
                            "B".to_string(),
                            "C".to_string()];
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
