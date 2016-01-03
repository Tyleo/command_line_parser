#[macro_export]
macro_rules! command_line {
    (
        $command_line_ident: ident {
            $(
                (
                    $field_ident: ident,
                    $field_type: ty,
                    $field_flag: expr
                )
            ),*
        }
    ) => {
        use command_line_parser::CommandLineParser;
        use std::iter::IntoIterator;
        use std::collections::vec_deque::VecDeque;

        pub struct $command_line_ident {
            pub this_program_name: String,
            $(
                pub $field_ident: Option<$field_type>,
            )*
        }

        impl $command_line_ident {
            pub fn parse<TIter>(args: TIter) -> Self
                where TIter: IntoIterator<Item = String> {
                let mut args_queue = args.into_iter()
                                         .collect();
                <Self as CommandLineParser>::parse(&mut args_queue)
            }
        }

        impl CommandLineParser for $command_line_ident {
            fn parse(args: &mut VecDeque<String>) -> Self {
                let this_program_name = {
                    match args.pop_front() {
                        Some(arg) => {
                            arg
                        },
                        None => {
                            panic!("Error, no program name was passed in as an argument on the command line.");
                        },
                    }
                };

                $(
                    let mut $field_ident: Option<$field_type> = None;
                )*

                while !args.is_empty() {
                    match args.pop_front() {
                        Some(arg) => {
                            match arg.as_ref() {
                                $(
                                    $field_flag => {
                                        $field_ident = Some(<$field_type as CommandLineParser>::parse(args));
                                    },
                                )*
                                _ => {
                                    panic!("Error matching flag: {}", arg);
                                }
                            }
                        },
                        None => {
                            panic!("Error popping arg queue in CommandLineParser. Queue is probably empty.");
                        }
                    }
                }

                $command_line_ident {
                    this_program_name: this_program_name,
                    $(
                        $field_ident: $field_ident
                    ),*
                }
            }
        }
    }
}
