use std::collections::{vec_deque, VecDeque};

pub struct CommandHistory {
    queue: VecDeque<String>,
    size: usize,
}

impl CommandHistory {
    pub fn new(size: usize) -> Self {
        Self { queue: VecDeque::with_capacity(size), size }
    }

    pub fn push(&mut self, value: impl Into<String>) {
        let value = value.into();
        if value == "history" || self.queue.back().map(|back| back == &value).unwrap_or(false) {
            return;
        }

        // Pop first so we only ever need to have space for N items allocated.
        if self.queue.len() == self.size {
            self.queue.pop_front();
        }

        self.queue.push_back(value);
    }

    pub fn iter<'a>(&'a self) -> vec_deque::Iter<'a, String> {
        self.queue.iter()
    }

    pub fn entry<'a>(&'a self, idx: usize) -> Option<&'a str> {
        self.queue.get(self.len() - 1 - idx).map(String::as_str)
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn command_history() {
        const SIZE: usize = 5;

        let mut history = CommandHistory::new(SIZE);
        for i in 0..SIZE + 1 {
            history.push(format!("command-{}", i + 1));
        }

        // "history" should not be pushed
        history.push("history");

        // Repeat value should not be pushed
        history.push("command-6");

        let values: Vec<_> = history.iter().map(String::as_str).collect();
        assert_eq!(&values, &["command-2", "command-3", "command-4", "command-5", "command-6"]);
    }
}
