use command_line_parser::CommandLineParser;
use std::collections::vec_deque::VecDeque;
use std::str::FromStr;

impl CommandLineParser for bool {
    fn parse(args: &mut VecDeque<String>) -> Self {
        let arg = match args.pop_front() {
            Some(arg) => {
                arg
            },
            None => {
                panic!("Error popping arg queue in bool CommandLineParser. Queue is probably empty.");
            },
        };

        let result = match bool::from_str(arg.as_ref()) {
            Ok(result) => {
                result
            },
            Err(err) => {
                panic!("Error parsing arg in bool CommandLineParser: {}", err);
            },
        };

        result
    }
}

impl CommandLineParser for String {
    fn parse(args: &mut VecDeque<String>) -> Self {
        let arg = match args.pop_front() {
            Some(arg) => {
                arg
            },
            None => {
                panic!("Error popping arg queue in bool CommandLineParser. Queue is probably empty.");
            },
        };

        arg
    }
}

impl <T0, T1> CommandLineParser for (T0, T1)
    where T0: CommandLineParser,
          T1: CommandLineParser {
    fn parse(args: &mut VecDeque<String>) -> Self {
        (T0::parse(args), T1::parse(args))
    }
}
