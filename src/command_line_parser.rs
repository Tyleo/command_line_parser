use std::collections::vec_deque::VecDeque;

pub trait CommandLineParser {
    fn parse(args: &mut VecDeque<String>) -> Self;
}
