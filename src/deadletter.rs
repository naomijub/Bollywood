use std::collections::VecDeque;

pub struct DeadLetter<T> {
    letter: VecDeque<T>,
}

impl<T> DeadLetter<T> {
    pub fn new() -> Self {
        Self {
            letter: VecDeque::new(),
        }
    }

    pub fn post(&mut self, letter: T) {
        self.letter.push_back(letter);
        // write to disk
    }
}
