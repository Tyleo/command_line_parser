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

impl CommandLineParser for usize {
    fn parse(args: &mut VecDeque<String>) -> Self {
        let arg = match args.pop_front() {
            Some(arg) => {
                arg
            },
            None => {
                panic!("Error popping arg queue in u8 CommandLineParser. Queue is probably empty.");
            },
        };

        let result = match usize::from_str(arg.as_ref()) {
            Ok(result) => {
                result
            },
            Err(err) => {
                panic!("Error parsing arg in u8 CommandLineParser: {}", err);
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

impl <T> CommandLineParser for Vec<T>
    where T: CommandLineParser {
    fn parse(args: &mut VecDeque<String>) -> Self {
        let len = usize::parse(args);
        [0..len].iter()
                .map(
                    |_| {
                        T::parse(args)
                    }
                )
                .collect()
    }
}

impl <T0, T1> CommandLineParser for (T0, T1)
    where T0: CommandLineParser,
          T1: CommandLineParser {
    fn parse(args: &mut VecDeque<String>) -> Self {
        (T0::parse(args), T1::parse(args))
    }
}
